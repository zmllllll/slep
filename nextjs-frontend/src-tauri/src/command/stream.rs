use payload::resources::{stream::StreamSettings, stream_level::StreamLevel};

use super::*;

#[tauri::command]
pub(crate) async fn update_stream_settings(
    gid: &str,
    stream: String,
    des: Option<String>,
    rlevel: i16,
    wlevel: i16,
) -> Result<(), Error> {
    let stream_settings = StreamSettings::new(des, rlevel, wlevel);
    let stream_action = GeneralAction::Upsert {
        id: Some((stream, gid.transform()?)),
        resource: stream_settings,
    };
    let stream_settings = Resources::StreamSettings(Command::new(
        gen_id().await,
        stream_action,
        "UpdateStreamSettings".to_string(),
    ));
    let cmds = Commands::Single(stream_settings);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_stream_settings: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn update_stream_level(
    gid: &str,
    stream: &str,
    rlevel: i16,
    wlevel: i16,
) -> Result<(), Error> {
    let stream_level = StreamLevel::new(rlevel, wlevel);
    let stream_action = GeneralAction::Upsert {
        id: Some((stream.to_string(), gid.transform()?)),
        resource: stream_level,
    };
    let stream_settings = Resources::StreamLevel(Command::new(
        gen_id().await,
        stream_action,
        "UpdateStreamLevel".to_string(),
    ));
    let cmds = Commands::Single(stream_settings);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_stream_level: {text}");
    send(TungMessage::Text(text)).await
}

pub mod query {
    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct StreamSettings {
        stream: String,
        des: Option<String>,
        rlevel: i16,
        wlevel: i16,
    }

    #[tauri::command]
    pub(crate) async fn get_stream_list(gid: &str) -> Result<String, Error> {
        let gid = gid.transform()?;
        StreamSettings::query(async move |pool| {
            let sql = "SELECT stream, des, rlevel, wlevel FROM stream_settings 
        WHERE gid = $1;";
            sqlx::query_as(sql).bind(gid).fetch_all(pool).await
        })
        .await?
        .serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_stream_settings() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        update_stream_settings(
            "7030073914839805946",
            "单元测试stream".to_string(),
            Some("测试".to_string()),
            1,
            2,
        )
        .await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "UpdateStreamSettings" => {
                            assert_eq!(event, "UpdateStreamSettings");
                            return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_stream_level() {
        let storage =
            PathBuf::from(r"C:\Users\Administrator\AppData\Local\com.tauri.nextjs.slep\storage");
        crate::STORAGE.set(storage).unwrap();

        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(15959119437, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        update_stream_level("7039165602011033594", "测试stream", 5, 6).await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "UpdateStreamLevel" => {
                            assert_eq!(event, "UpdateStreamLevel");
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
