use payload::resources::{topic::TopicSettings, topic_level::TopicLevel};

use super::*;

#[tauri::command]
pub(crate) async fn update_topic_settings(
    gid: Option<&str>,
    stream: &str,
    topic: &str,
    associate_task_id: Option<&str>,
    rlevel: i16,
    wlevel: i16,
) -> Result<(), Error> {
    // FIXME:
    return Err(Error::System("todo".to_string()));

    let topic_settings = TopicSettings::new(associate_task_id.transform()?, rlevel, wlevel);
    let hash_key = gen_topic_key(gid.transform()?, stream, topic);
    let task_action = GeneralAction::Update {
        id: hash_key,
        resource: topic_settings,
    };
    let topic_settings = Resources::TopicSettings(Command::new(
        gen_id().await,
        task_action,
        "UpdateTopicSettings".to_string(),
    ));
    let cmds = Commands::Single(topic_settings);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_topic_settings: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn update_topic_level(
    gid: Option<&str>,
    stream: &str,
    topic: &str,
    rlevel: i16,
    wlevel: i16,
) -> Result<(), Error> {
    let topic_level = TopicLevel::new(rlevel, wlevel);
    let hash_key = gen_topic_key(gid.transform()?, stream, topic);
    let task_action = GeneralAction::Update {
        id: hash_key,
        resource: topic_level,
    };
    let topic_settings = Resources::TopicLevel(Command::new(
        gen_id().await,
        task_action,
        "UpdateTopicSettings".to_string(),
    ));
    let cmds = Commands::Single(topic_settings);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_topic_settings: {text}");
    send(TungMessage::Text(text)).await
}

pub mod query {
    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct TopicSettings {
        associate_task_id: Option<i64>,
        rlevel: i16,
        wlevel: i16,
    }

    #[tauri::command]
    pub(crate) async fn get_topic_settings(
        gid: Option<&str>,
        stream: &str,
        topic: &str,
    ) -> Result<String, Error> {
        let gid = gid.transform()?;
        TopicSettings::query(async move |pool| {
            let hashkey = gen_topic_key(gid, stream, topic);
            let sql = "SELECT associate_task_id, rlevel, wlevel FROM topic_settings 
            WHERE hashkey = $1;";
            sqlx::query_as(sql).bind(hashkey).fetch_all(pool).await
        })
        .await?
        .pop()
        .serde_to_string()
    }

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct Topic {
        topic: String,
        timestamp: i64,
    }

    #[tauri::command]
    pub(crate) async fn get_topic_list(
        gid: Option<&str>,
        stream: Option<&str>,
        receiver: Option<&str>,
    ) -> Result<String, Error> {
        let addr = match (gid, stream, receiver) {
            (None, None, Some(receiver)) => receiver,
            (Some(gid), Some(stream), None) => stream,
            _ => {
                return Err(Error::BadRequest(
                    "receiver & (gid, stream) must have one".to_string(),
                ))
            }
        };

        Topic::query(async move |pool| {
            let sql = "SELECT topic, max(timestamp) as timestamp FROM message WHERE gid = $1 AND addr = $2 GROUP BY topic ORDER BY timestamp DESC;";
            sqlx::query_as(sql).bind(gid).bind(addr).fetch_all(pool).await
        })
        .await?
        .serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_topic_level() {
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
        update_topic_level(Some("7039165602011033594"), "测试stream", "测试topic", 5, 6).await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "UpdateTopicLevel" => {
                            assert_eq!(event, "UpdateTopicLevel");
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_topic_list() {
        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://18552513141.db")
            .await
            .unwrap();
        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let res = crate::command::topic::query::get_topic_list(
            Some("7039543453465980922"),
            Some("slep"),
            None,
        )
        .await;
        println!("res: {res:#?}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_topic_settings() {
        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://test.db")
            .await
            .unwrap();
        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let res = crate::command::topic::query::get_topic_settings(
            Some("7036997188354060265"),
            "c1",
            "t1",
        )
        .await;
        println!("res: {res:#?}");
    }
}
