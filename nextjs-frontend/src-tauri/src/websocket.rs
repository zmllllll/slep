use std::borrow::Cow;

use futures::StreamExt as _;
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

use crate::notify::Notifies;

use super::*;

mod stream;

enum LoginStatus {
    LogOut,
    Error,
}

pub(crate) async fn connect(uid: i64, uuid: &str) -> Result<()> {
    info!("[LoopConnect] wait");
    let (db, url) = init::config::read_configure();
    let ws_url = format!("ws://{}/login/{}/{}", &url, uid, uuid);
    {
        let mut a = CLIENT.lock().await;
        a.log_status = false;
    }
    loop {
        {
            let client = CLIENT.lock().await;
            if client.log_status {
                Notifies::data("LogOut", None).send();
                info!("[LoopConnect] log out when disconnected");
                break;
            }
        }
        match loop_connect(&ws_url, uid, &db).await {
            Ok(status) => match status {
                LoginStatus::LogOut => {
                    Notifies::data("LogOut", None).send();
                    info!("[LoopConnect] log out when connected");
                    break;
                }
                LoginStatus::Error => {
                    warn!("[LoopConnect] erroooooooo");
                    continue;
                }
            },
            Err(e) => {
                error!("[LoopConnect] connect error: {}", e);
                if let Some(d_err) = e.downcast_ref::<tungstenite::error::Error>() {
                    match d_err {
                        tungstenite::Error::Http(resp) => {
                            if resp.status() == 401 {
                                return Err(e);
                            }
                        }
                        _ => continue,
                    }
                };
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        warn!("[LoopConnect] reconnecting....");
        Notifies::data("Reconnect", None).send();
    }
    Ok(())
}

async fn loop_connect(ws_url: &str, uid: i64, db: &init::config::DB) -> Result<LoginStatus> {
    let socket = stream::websocket_stream(ws_url).await?;
    let (mut writer, mut reader) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<TungMessage>();

    init::init(uid, db).await?;
    info!("init successful!");

    ping(tx.clone(), uid);
    write(writer, rx);
    read(reader, tx).await
}

async fn read(
    mut reader: futures::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
    >,
    tx: UnboundedSender<TungMessage>,
) -> Result<LoginStatus> {
    use futures::SinkExt as _;
    {
        let mut sender = CHANNEL_SENDER.write().await;
        sender.0 = Some(tx.clone());
        drop(sender);
    }

    while let Some(msg) = reader.next().await {
        match msg {
            Ok(msg) => {
                // if cfg!(debug_assertions) {}
                info!("[StreamReader] receive websocket message from server: {msg:?}");

                match msg {
                    TungMessage::Text(t) => {
                        match serde_json::from_str::<Commands<resources::Resources>>(&t) {
                            Ok(cmds) => {
                                tokio::task::spawn_blocking(move || {
                                    tokio::runtime::Handle::current().block_on(async move {
                                        if let Err(e) = cmds
                                            .execute(unsafe { SQLITE_POOL.get().unwrap() })
                                            .await
                                        {
                                            error!("[StreamReader] sqlite execute error: {e}");
                                        };
                                        use report::Reporter as _;
                                        tokio::time::sleep(tokio::time::Duration::from_millis(500))
                                            .await;
                                        cmds.handle().send();
                                    });
                                });
                            }
                            Err(e) => {
                                error!("[StreamReader] deserialize text to report error: {e}");
                                return Err(error::Error::System(e.to_string()).into());
                            }
                        };
                    }
                    TungMessage::Binary(_) => todo!(),
                    TungMessage::Ping(p) => {
                        // debug!("get ping");

                        // tx.send(TungMessage::Pong(p)).unwrap_or_else(|e| {
                        //     error!("channel send pong err:{}", e);
                        // });
                    }
                    TungMessage::Pong(pong) => {
                        // info!(
                        //     "get pong: {:?}",
                        //     String::from_utf8(pong).expect("Found invalid UTF-8")
                        // );
                    }
                    TungMessage::Close(frame) => {
                        warn!("*********************** passively close ***********************");
                        info!("[StreamReader] passively closing connection...");
                        info!(
                            "[StreamReader] ready to send a close message, close frame: {frame:?}"
                        );
                        if let Some(frame) = &frame {
                            match &frame.reason {
                                Cow::Borrowed(b) => {}
                                Cow::Owned(o) => match o.as_str() {
                                    "log out" => {
                                        warn!("***********************   disconnected   ***********************");
                                        return Ok(LoginStatus::LogOut);
                                    }
                                    _ => {}
                                },
                            }
                        }
                        tx.send(TungMessage::Close(frame)).unwrap_or_else(|e| {
                            error!("[StreamReader] channel send close message error:{}", e);
                        });
                        warn!("***********************   disconnected   ***********************");
                        break;
                    }
                    TungMessage::Frame(_) => todo!(),
                }
            }
            Err(e) => {
                error!("[StreamReader] reading websocket message error: {}", e);
                tx.send(TungMessage::Close(Some(CloseFrame {
                    code: CloseCode::Error,
                    reason: Cow::Owned(e.to_string()),
                })))
                .unwrap_or_else(|e| {
                    error!("[StreamReader] channel send close message error:{}", e);
                });
                return Err(error::Error::System(e.to_string()).into());
            }
        }
    }
    Ok(LoginStatus::Error)
}

fn write(
    mut writer: futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
        TungMessage,
    >,
    mut rx: UnboundedReceiver<TungMessage>,
) {
    use futures::{SinkExt as _, TryFutureExt as _};
    tokio::task::spawn(async move {
        loop {
            let cmd = match rx.recv().await {
                Some(m) => m,
                None => {
                    error!("[StreamWriter] receive none from channel");
                    continue;
                }
            };
            match cmd {
                TungMessage::Text(t) => {
                    info!("[StreamWriter] receive text from channel: {:?}", t);
                    writer
                        .send(TungMessage::Text((t).to_string()))
                        .unwrap_or_else(|e| {
                            error!("[StreamWriter] send {t} error: {}", e);
                        })
                        .await;
                }
                TungMessage::Binary(_) => todo!(),
                TungMessage::Ping(p) => {
                    writer
                        .send(TungMessage::Ping(p))
                        .unwrap_or_else(|e| {
                            error!("[StreamWriter] send ping message error:{}", e);
                        })
                        .await;
                }
                TungMessage::Pong(p) => {}
                TungMessage::Close(frame) => {
                    warn!("***********************  actively close  ***********************");
                    info!("[StreamWriter] ready to send a close message, close frame: {frame:?}");
                    info!("[StreamWriter] actively closing connection...");
                    writer
                        .send(TungMessage::Close(frame))
                        .unwrap_or_else(|e| {
                            error!("[StreamWriter] send close message error:{}", e);
                        })
                        .await;
                    writer
                        .close()
                        .unwrap_or_else(|e| {
                            error!("[StreamWriter] close stream writer error:{}", e);
                        })
                        .await;
                    rx.close();
                    drop(rx);
                    warn!("***********************   disconnected   ***********************");
                    break;
                }
                TungMessage::Frame(_) => todo!(),
            }
        }
    });
}

fn ping(tx: UnboundedSender<TungMessage>, uid: i64) {
    tokio::task::spawn(async move {
        loop {
            if let Err(e) = tx.send(TungMessage::Ping(
                format!("[{uid}] ping").as_bytes().to_vec(),
            )) {
                error!("send ping message error:{}", e);
                return;
            };
            tokio::time::sleep(tokio::time::Duration::from_secs(237)).await;

            // tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
        }
    });
}
