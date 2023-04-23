use payload::resources::{message::Message, task_receipt::TaskReceipt};

use super::*;

#[tauri::command]
pub(crate) async fn add_task_receipt(
    hashkey: &str,
    task_id: &str,
    executor: &str,
    status: String,
    des: Option<String>,
) -> Result<(), Error> {
    let (receipt, message) = _add_task_receipt(
        hashkey.transform()?,
        task_id.transform()?,
        executor.transform()?,
        status,
        des,
        None,
    )
    .await?;

    let cmds = Commands::Multi(vec![receipt, message]);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("add_task_receipt: {text}");
    send(TungMessage::Text(text)).await
}

pub(crate) async fn _add_task_receipt(
    hashkey: i64,
    task_id: i64,
    executor: i64,
    status: String,
    des: Option<String>,
    hashkey_break: Option<query::HashkeyBreak>,
) -> Result<(Resources, Resources), Error> {
    let task = TaskReceipt::new(hashkey, executor, status, des.clone());

    let task_action = GeneralAction::Update {
        id: task_id,
        resource: task,
    };
    let receipt = Resources::TaskReceipt(Command::new(
        gen_id().await,
        task_action,
        "AddTaskReceipt".to_string(),
    ));
    let query::HashkeyBreak { gid, addr, topic } = if let Some(hashkey_break) = hashkey_break {
        hashkey_break
    } else {
        query::get_hashkey_break(hashkey.to_string().as_str()).await?
    };
    let uid = user::SELF
        .read()
        .await
        .0
        .ok_or(Error::System("user do not exist".to_string()))
        .inspect_err(|e| tracing::error!("assign_task get self uid error: {e:?}"))?;
    // let uid = user::SELF
    //     .get()
    // .ok_or(Error::System("user do not exist".to_string()))
    // .inspect_err(|e| tracing::error!("assign_task get self uid error: {e:?}"))?;

    let message = message::_send_message(
        gid,
        Some(&addr),
        topic.to_string(),
        "bot".to_string(),
        des.clone().unwrap_or_default(),
        uid,
        None,
    )
    .await?;
    Ok((receipt, message))
}

pub mod query {
    use std::collections::HashMap;

    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct HashkeyBreak {
        pub(crate) gid: Option<i64>,
        pub(crate) addr: String,
        pub(crate) topic: String,
    }

    pub(crate) async fn get_hashkey_break(hashkey: &str) -> Result<HashkeyBreak, Error> {
        let hashkey = hashkey.transform()?;
        tracing::info!("hashkey: {hashkey}");
        HashkeyBreak::query(async move |pool| {
            let sql = "SELECT gid, addr, topic FROM message GROUP BY gid, addr, topic";
            sqlx::query_as(sql).fetch_all(pool).await
        })
        .await?
        .map(|hashkeys| {
            match hashkeys
                .into_iter()
                .find(|hk| gen_topic_key(hk.gid, &hk.addr, &hk.topic) == hashkey)
            {
                Some(h) => Ok(h),
                None => Err(Error::System(format!("hashkey [{hashkey}] not exist"))),
            }
        })
    }

    pub(crate) async fn query_sqlite_hashkey_break_with_gid(
        gid: i64,
    ) -> Result<Vec<HashkeyBreak>, Error> {
        HashkeyBreak::query(async move |pool| {
            let sql =
                "SELECT gid, addr, topic FROM message WHERE gid = $1 GROUP BY gid, addr, topic";
            sqlx::query_as(sql).bind(gid).fetch_all(pool).await
        })
        .await
    }

    pub(crate) async fn query_all_hashkey_break(
        pg: &sqlx::Pool<sqlx::Any>,
    ) -> anyhow::Result<HashMap<i64, HashkeyBreak>> {
        let sql = "SELECT gid, addr, topic FROM slep.message";
        let temps: Vec<HashkeyBreak> = sqlx::query_as(sql).fetch_all(pg).await?;
        Ok(temps
            .into_iter()
            .map(|temp| {
                let hk = gen_topic_key(temp.gid, &temp.addr, &temp.topic);
                (hk, temp)
            })
            .collect::<HashMap<i64, HashkeyBreak>>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_add_task_receipt() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        add_task_receipt(
            "-7978900843280652265",
            "7030144804906610682",
            "1606174953750073345",
            "created".to_string(),
            Some("回执测试".to_string()),
        )
        .await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "AddTaskReceipt" => {
                            assert_eq!(event, "AddTaskReceipt");
                            return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_query_all_hashkey_break() {
        let pg = sqlx::Pool::<sqlx::Any>::connect(
            "postgresql://1.15.14.20?dbname=slep&user=postgres&password=quake@123",
        )
        .await
        .unwrap();

        let res = query::query_all_hashkey_break(&pg).await;
        println!("res: {res:#?}");
    }
}
