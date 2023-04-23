use crate::resources::Resources;

use super::*;
use payload::{error, resources::message::Message};

#[tauri::command]
pub(crate) async fn send_message(
    gid: Option<&str>,
    stream: Option<&str>,
    topic: String,
    message_type: String,
    content: String,
    sender: &str,
    receiver: Option<&str>,
) -> Result<(), Error> {
    if let Some(gid) = gid {
        let query: Vec<query::Levels> =
            query::sqlite_levels_query(gid.transform()?, sender.transform()?).await?;
        if let Some(level) = query.first()
        && level.member_level > level.stream_wlevel{
            return Err(error::Error::InsufficientPermissions(level.member_level, level.stream_wlevel))
        }
    }
    let message = _send_message(
        gid.transform()?,
        stream,
        topic,
        message_type,
        content,
        sender.transform()?,
        receiver,
    )
    .await?;
    let cmds = Commands::Single(message);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("send_message: {text}");
    send(TungMessage::Text(text)).await
}

pub(crate) async fn _send_message(
    gid: Option<i64>,
    stream: Option<&str>,
    topic: String,
    message_type: String,
    content: String,
    sender: i64,
    receiver: Option<&str>,
) -> Result<Resources, Error> {
    let addr = match (gid, stream, receiver) {
        (None, None, Some(receiver)) => receiver,
        (Some(gid), Some(stream), None) => stream,
        _ => {
            return Err(Error::BadRequest(
                "receiver & (gid, stream) must have one".to_string(),
            ))
        }
    };

    let message = Message::new(gid, message_type, addr.to_string(), topic, content, sender);

    let message_action = GeneralAction::Upsert {
        id: Some(gen_id().await),
        resource: message,
    };

    Ok(Resources::Message(Command::new(
        gen_id().await,
        message_action,
        "SendMessage".to_string(),
    )))
}

#[tauri::command]
pub(crate) async fn revoke_message(id: &str) -> Result<(), Error> {
    let message_action = GeneralAction::Drop(id.transform()?);

    let message = Resources::Message(Command::new(
        gen_id().await,
        message_action,
        "RevokeMessage".to_string(),
    ));
    let cmds = Commands::Single(message);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("revoke_message: {text}");
    send(TungMessage::Text(text)).await
}

// #[tauri::command]
// pub(crate) async fn test_delete_message(message_id: &str) -> Result<(), Error> {
//     let message_action = GeneralAction::Drop(message_id.transform()?);

//     let message = Resources::Message(Command::new(gen_id().await, message_action).await);
//     let cmds = Commands::Single(message);
//     let text = serde_json::to_string(&cmds).unwrap();
//     tracing::info!("delete_message: {text}");
//     send(text)
// }

pub mod query {
    use crate::command::user::SELF;

    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct Levels {
        gid: i64,
        uid: i64,
        pub(crate) member_level: i16,
        pub(crate) stream_rlevel: i16,
        pub(crate) stream_wlevel: i16,
    }

    pub(crate) async fn sqlite_levels_query(gid: i64, uid: i64) -> Result<Vec<Levels>, Error> {
        Levels::query(async move |pool| {
            let sql = "SELECT M.gid, uid, M.level member_level, 
            S.rlevel stream_rlevel, S.wlevel stream_wlevel FROM group_member M 
            JOIN stream_settings S ON M.gid = S.gid WHERE M.gid =$1 AND M.uid =$2;";
            sqlx::query_as(sql)
                .bind(gid)
                .bind(uid)
                .fetch_all(pool)
                .await
        })
        .await
    }

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct Message {
        id: i64,
        gid: Option<i64>,
        typ: String,
        addr: String,
        topic: String,
        content: String,
        sender: i64,
        #[sqlx(rename = "name")]
        username: String,
        timestamp: i64,
    }

    #[tauri::command]
    pub(crate) async fn get_private_message(uid: &str) -> Result<String, Error> {
        let uid = uid.transform()?;
        Message::query(async move |pool| {
            let sql = "SELECT M.id, gid, typ, addr, topic, content, sender, name, M.timestamp  
            FROM message M JOIN user U ON sender = U.id 
        WHERE gid = NULL AND (addr = $1 OR sender = $1);";
            sqlx::query_as(sql).bind(uid).fetch_all(pool).await
        })
        .await?
        .serde_to_string()
    }

    #[tauri::command]
    pub(crate) async fn get_stream_message(gid: &str, stream: &str) -> Result<String, Error> {
        let gid = gid.transform()?;
        let uid = SELF
            .read()
            .await
            .0
            .ok_or(error::Error::System("User ID Not Exist".to_string()))?;
        tracing::error!("[get_stream_message]: {uid}");
        let query: Vec<query::Levels> = query::sqlite_levels_query(gid, uid).await?;
        if let Some(level) = query.first()
        && level.member_level > level.stream_rlevel{
            return Err(error::Error::InsufficientPermissions(level.member_level, level.stream_rlevel))
        }
        tracing::error!("[get_stream_message] query: {query:#?}");

        Message::query(async move |pool| {
            let sql = "SELECT M.id, gid, typ, addr, topic, content, sender, name, M.timestamp  
            FROM message M JOIN user U ON sender = U.id
        WHERE gid = $1 AND addr = $2;";
            sqlx::query_as(sql)
                .bind(gid)
                .bind(stream)
                .fetch_all(pool)
                .await
        })
        .await?
        .serde_to_string()
    }

    #[tauri::command]
    pub(crate) async fn get_topic_message(
        gid: &str,
        stream: &str,
        topic: &str,
    ) -> Result<String, Error> {
        let gid = gid.transform()?;
        println!("gid: {gid}, stream: {stream}, topic: {topic}");
        Message::query(async move |pool| {
            let sql = "SELECT M.id, gid, typ, addr, topic, content, sender, name, M.timestamp  
        FROM message M JOIN user U ON sender = U.id
        WHERE gid = $1 AND addr = $2 AND topic = $3;";
            sqlx::query_as(sql)
                .bind(gid)
                .bind(stream)
                .bind(topic)
                .fetch_all(pool)
                .await
        })
        .await?
        .serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_send_message() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        send_message(
            Some("7029392459348324346"),
            Some("qwj_stream"),
            "qwj_topic".to_string(),
            "md".to_string(),
            "看就看见艰苦".to_string(),
            "1606174953750073345",
            None,
        )
        .await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "SendMessage" => {
                            assert_eq!(event, "SendMessage");
                            return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_revoke_message() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        revoke_message("1624329519984988160").await;
        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "RevokeMessage" => {
                            assert_eq!(event, "RevokeMessage");
                            return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_private_message() {
        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://123123.db")
            .await
            .unwrap();
        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let res = crate::command::message::query::get_private_message("7032557059040358394").await;
        println!("res: {res:#?}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_stream_message() {
        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://123123.db")
            .await
            .unwrap();
        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let res =
            crate::command::message::query::get_stream_message("7036997188354060265", "c1").await;
        println!("res: {res:#?}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_topic_message() {
        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://test.db")
            .await
            .unwrap();
        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let res =
            crate::command::message::query::get_topic_message("7036997188354060265", "c1", "t1")
                .await;
        println!("res: {res:#?}");
    }
}
