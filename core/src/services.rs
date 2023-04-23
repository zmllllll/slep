use super::*;
use axum::response::IntoResponse;

pub(super) async fn login(
    axum::extract::Path((username, uuid)): axum::extract::Path<(i64, String)>,
    ws: axum::extract::ws::WebSocketUpgrade,
    axum::Extension(collect_tx): axum::extract::Extension<TokioUnboundedSender<Event>>,
) -> impl axum::response::IntoResponse {
    use axum::extract::ws::Message;
    let (ok, user) = check_uid(username, crate::PG_POOL.get().unwrap()).await;
    let user_task = if ok && let Some(user) = user{
        let task = resource::Commands::Single(resources::Resources::Username(
            resource::Command::new(
                0,
                resource::GeneralAction::Upsert {
                    id: Some(username),
                    resource: payload::resources::username::Username::new(user.name),
                },
                "Login".to_string(),
            ),
        ));
        serde_json::to_string(&task).unwrap()
    } else {
        return (axum::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response();
    };
    // #[derive(Debug, Deserialize, Serialize, Default)]
    // pub(crate) struct CheckToken {
    //     #[serde(rename(deserialize = "name"))]
    //     pub(crate) username: String,
    // }

    // let url = String::from("https://1to2to3.cn/super-login/sys/me");
    // let resp = reqwest::Client::new()
    //     .get(url)
    //     .header("Authorization", token)
    //     .send()
    //     .await
    //     .unwrap();
    // let username = if resp.status().is_success() {
    //     match resp.json::<CheckToken>().await {
    //         Ok(json) => json.username,
    //         Err(_) => return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response(),
    //     }
    // } else {
    //     return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response();
    // };
    // let username = token;
    tracing::info!("`{username}[{uuid}]` connected");

    ws.on_upgrade(async move |socket| {
        use futures_util::StreamExt as _;
        let (mut user_ws_tx, mut user_ws_rx) = socket.split();
        let (sender, rx) = tokio::sync::mpsc::unbounded_channel();
        let mut rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

        if let Err(_disconnected) =
            collect_tx.send(Event::Connect(username, uuid.clone(), sender.clone()))
        {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
        }

        let collect_tx_c = collect_tx.clone();

        use futures_util::SinkExt as _;
        use futures_util::TryFutureExt as _;
        user_ws_tx
            .send(Message::Text(user_task))
            .unwrap_or_else(|e| {
                tracing::error!("[WS] text send to client `{username}` error: {}", e);
            })
            .await;

        tokio::task::spawn(async move {
            while let Some(result) = user_ws_rx.next().await {
                let msg = match result {
                    Ok(msg) => msg,
                    Err(e) => {
                        tracing::error!("[WS] `{username}`receives msg from client error: {e}");
                        if let Err(e) = sender.send(ConnectionEvent::Disconnect) {
                            tracing::error!("[WS] `{username}`disconnect send error: {e}");
                            break;
                        }
                        break;
                    }
                };
                // handle client message
                match msg {
                    Message::Text(cmds) => {
                        let cmd: Result<resource::Commands<crate::resources::Resources>, _> =
                            serde_json::from_str(&cmds);

                        // let cmd: Result<payload::command::Command, _> = serde_json::from_str(&cmds);

                        if let Ok(cmd) = cmd {
                            tracing::info!("[WS] `{username}` receives msg from client: {cmd:?}");
                            if let Err(err) =
                                collect_tx_c.send(Event::Resource(username, Box::new(cmd), None))
                            {
                                tracing::error!("[WS] `{username}`database op send err: {:?}", err);
                            } else {
                                tracing::info!("?????");
                            };
                        }
                    }
                    Message::Binary(_) => {}
                    Message::Ping(ping) => {
                        tracing::info!(
                            "get ping: {:?}",
                            String::from_utf8(ping).expect("Found invalid UTF-8")
                        );
                    }
                    Message::Pong(pong) => {
                        tracing::info!("get pong");
                        let _ = collect_tx_c.clone().send(Event::Pong(username, pong));
                    }
                    Message::Close(c) => {
                        tracing::info!("[WS] `{username}`received CLOSE signal from client: {c:?}");
                        if let Err(e) = sender.send(ConnectionEvent::Disconnect) {
                            tracing::error!("[WS] `{username}`disconnect send error: {e}");
                            break;
                        }
                        tracing::info!("[WS] `{username}`closing......");
                    }
                }
            }
        });

        while let Some(sessions) = rx.next().await {
            match sessions {
                ConnectionEvent::Message(task) => {
                    tracing::info!("[WS]  ready to send msg to client `{username}`: {task:?}");
                    user_ws_tx
                        .send(Message::Text(task))
                        .unwrap_or_else(|e| {
                            tracing::error!("[WS] text send to client `{username}` error: {}", e);
                        })
                        .await;
                }
                ConnectionEvent::Disconnect => {
                    use axum::extract::ws::{close_code, CloseFrame};

                    user_ws_tx
                        .send(Message::Close(Some(CloseFrame {
                            code: close_code::AWAY,
                            reason: String::from("going away close").into(),
                        })))
                        .unwrap_or_else(|e| {
                            tracing::error!("disconnect send error: {}", e);
                        })
                        .await;
                    // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    let _ = collect_tx
                        .clone()
                        .send(Event::Disconnect(username, uuid.clone()));
                    rx.close();
                    break;
                }
                ConnectionEvent::Ping(b) => {
                    user_ws_tx
                        .send(Message::Ping(b))
                        .unwrap_or_else(|e| {
                            let _ = collect_tx.send(Event::Disconnect(username, uuid.clone()));
                            rx.close();
                            tracing::error!("ping send error: {}", e);
                        })
                        .await;
                    // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }

        // TODO:
        // user_ws_rx stream will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        // user_disconnected(&username, &uuid).await;
        tracing::info!("disconnecting");
    })
}

async fn check_uid(uid: i64, pg: &sqlx::Pool<sqlx::Any>) -> (bool, Option<user::UserInfo>) {
    let sql = format!("SELECT id, name, profile_picture FROM slep.user WHERE id ={uid};");
    let mut user: Vec<user::UserInfo> = sqlx::query_as(&sql).fetch_all(pg).await.unwrap();
    (!user.is_empty(), user.pop())
}
