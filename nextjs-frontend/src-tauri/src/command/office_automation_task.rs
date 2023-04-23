use payload::resources::{
    message::Message, office_automation_task::OATask, task_id::TaskId, task_receipt::TaskReceipt,
};

use super::*;

#[tauri::command]
pub(crate) async fn assign_task(
    gid: Option<&str>,
    stream: Option<&str>,
    receiver: Option<&str>,
    topic: &str,
    name: String,
    des: Option<String>,
    typ: String,
    consignor: &str,
    deadline: &str,
) -> Result<(), Error> {
    let uid = user::SELF
        .read()
        .await
        .0
        .ok_or(Error::System("user do not exist".to_string()))
        .inspect_err(|e| tracing::error!("assign_task get self uid error: {e:?}"))?;
    let gid = gid.transform()?;

    // TODO: 优化逻辑， enum包
    let mut cmds = _assign_task(
        uid, gid, stream, receiver, topic, name, des, typ, consignor, deadline,
    )
    .await?;

    let cmds = Commands::Multi(cmds);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("assign_task: {text}");
    send(TungMessage::Text(text)).await
}

pub(crate) async fn _assign_task(
    uid: i64,
    gid: Option<i64>,
    stream: Option<&str>,
    receiver: Option<&str>,
    topic: &str,
    name: String,
    des: Option<String>,
    typ: String,
    consignor: &str,
    deadline: &str,
) -> Result<Vec<Resources>, Error> {
    let addr = match (gid, stream, receiver) {
        (None, None, Some(receiver)) => receiver,
        (Some(gid), Some(stream), None) => stream,
        _ => {
            return Err(Error::BadRequest(
                "receiver & (gid, stream) must have one".to_string(),
            ))
        }
    };

    let hashkey = gen_topic_key(gid, addr, topic);

    let not_duplicate = topic::query::TopicSettings::query(async move |pool| {
        let sql = "SELECT associate_task_id, rlevel, wlevel FROM topic_settings 
    WHERE hashkey = $1;";
        sqlx::query_as(sql).bind(hashkey).fetch_all(pool).await
    })
    .await?
    .is_empty();

    if !not_duplicate {
        return Err(Error::System("target topic already have task".to_string()));
    }

    let task_id = payload::resources::gen_id().await;

    let message = message::_send_message(
        gid,
        stream,
        topic.to_string(),
        "bot".to_string(),
        des.clone().unwrap_or_default(),
        uid,
        None,
    )
    .await?;
    let task = OATask::new(
        name,
        des,
        typ,
        consignor.transform()?,
        deadline.transform()?,
    );
    let task_action = GeneralAction::Insert {
        id: Some(task_id),
        resource: task,
    };
    let task = Resources::Task(Command::new(
        gen_id().await,
        task_action,
        "AssignTask".to_string(),
    ));

    let bind = _bind_task_into_topic(task_id, gid, addr, topic, hashkey).await?;

    let hashkey_break = task_receipt::query::HashkeyBreak {
        gid,
        addr: addr.to_string(),
        topic: topic.to_string(),
    };
    let (receipt, receipt_message) = task_receipt::_add_task_receipt(
        hashkey,
        task_id,
        uid,
        "created".to_string(),
        Some("created".to_string()),
        Some(hashkey_break),
    )
    .await?;

    Ok(vec![task, message, bind, receipt, receipt_message])
}

pub(crate) async fn _bind_task_into_topic(
    task_id: i64,
    gid: Option<i64>,
    addr: &str,
    topic: &str,
    hashkey: i64,
) -> Result<Resources, Error> {
    println!("hashkey: {hashkey}");

    let tid = TaskId::new(task_id, gid, addr.to_string(), topic.to_string());
    let topic_action = GeneralAction::Upsert {
        id: Some(hashkey),
        resource: tid,
    };
    Ok(Resources::TaskId(Command::new(
        gen_id().await,
        topic_action,
        "AddTaskToTopic".to_string(),
    )))
}

pub mod query {
    use std::collections::BTreeMap;

    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct QueryTask {
        pub(crate) id: i64,
        pub(crate) name: String,
        pub(crate) hashkey: i64,
        pub(crate) task_des: Option<String>,
        pub(crate) typ: String,
        pub(crate) consignor: i64,
        pub(crate) consignor_name: String,
        pub(crate) deadline: i64,
        pub(crate) task_timestamp: i64,
        pub(crate) receipts: Option<Vec<QueryReceipt>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct QueryReceipt {
        pub(crate) executor: Option<i64>,
        pub(crate) status: Option<String>,
        pub(crate) receipt_des: Option<String>,
        pub(crate) receipt_timestamp: Option<i64>,
        pub(crate) executor_name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct QueryTasks(BTreeMap<i64, QueryTask>);

    impl From<Vec<SqliteTask>> for QueryTasks {
        fn from(value: Vec<SqliteTask>) -> Self {
            let mut data = BTreeMap::<i64, QueryTask>::new();
            value.iter().for_each(|st| {
                let rs = if let Some(executor) = st.executor
                && let Some(status) = &st.status
                && let Some(receipt_des) = &st.receipt_des
                && let Some(receipt_timestamp) = st.receipt_timestamp{
                    Some(QueryReceipt {
                        executor: st.executor,
                        executor_name: st.executor_name.to_owned(),
                        status: st.status.to_owned(),
                        receipt_des: st.receipt_des.to_owned(),
                        receipt_timestamp: st.receipt_timestamp,
                    })
                }else{
                    None
                };

                data.entry(st.id)
                    .and_modify(|mut task| {
                        if let Some(receipts) = &mut task.receipts
                        && let Some(rs) = &rs{
                            receipts.push(rs.to_owned());
                        }
                    })
                    .or_insert(QueryTask {
                        id: st.id,
                        name: st.name.to_owned(),
                        hashkey: st.hashkey.to_owned(),
                        task_des: st.task_des.to_owned(),
                        typ: st.typ.to_owned(),
                        consignor: st.consignor,
                        deadline: st.deadline,
                        task_timestamp: st.task_timestamp,
                        receipts: rs.map(|rs| vec![rs]),
                        consignor_name: st.consignor_name.to_owned(),
                    });
            });
            QueryTasks(data)
        }
    }

    #[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
    pub(crate) struct SqliteTask {
        pub(crate) id: i64,
        pub(crate) name: String,
        pub(crate) hashkey: i64,
        pub(crate) task_des: Option<String>,
        pub(crate) typ: String,
        pub(crate) consignor: i64,
        pub(crate) consignor_name: String,
        pub(crate) deadline: i64,
        pub(crate) task_timestamp: i64,
        pub(crate) executor: Option<i64>,
        pub(crate) executor_name: Option<String>,
        pub(crate) status: Option<String>,
        pub(crate) receipt_des: Option<String>,
        pub(crate) receipt_timestamp: Option<i64>,
    }

    #[tauri::command]
    pub(crate) async fn get_task_list(gid: &str) -> Result<String, Error> {
        let gid = gid.transform()?;

        let hashkeys = task_receipt::query::query_sqlite_hashkey_break_with_gid(gid)
            .await?
            .into_iter()
            .filter(|hk| hk.gid == Some(gid))
            .map(|hk| gen_topic_key(hk.gid, &hk.addr, &hk.topic))
            .collect::<std::collections::HashSet<i64>>();
        let hashkeys = crate::init::pull::any_in_hashset(hashkeys, ",");
        SqliteTask::query(async move |pool| {
            let sql = format!("SELECT T.id, hashkey, T.name, T.des task_des, typ, consignor, TempU.name AS consignor_name, deadline, T.timestamp task_timestamp, 
            task_id, executor, u.name AS executor_name, status, TR.des receipt_des, TR.timestamp receipt_timestamp 
            FROM office_automation_task T 
            LEFT JOIN office_automation_task_receipt TR ON T.id = TR.task_id 
            JOIN user U ON u.id = TR.executor 
            JOIN user TempU ON TempU.id = consignor 
            JOIN topic_settings S ON T.id = S.associate_task_id 
            WHERE hashkey in({hashkeys});");
            sqlx::query_as(&sql).fetch_all(pool).await
        })
        .await?
        .map(Into::<QueryTasks>::into)
        .0.serde_to_string()
    }

    #[tauri::command]
    pub(crate) async fn get_task_list_by_consignor(consignor: &str) -> Result<String, Error> {
        let consignor = consignor.transform()?;
        SqliteTask::query(async move |pool| {
            let sql = "SELECT T.id, hashkey, T.name, T.des task_des, typ, consignor, deadline, T.timestamp task_timestamp, 
            task_id, executor, u.name AS executor_name, status, TR.des receipt_des, TR.timestamp receipt_timestamp 
            FROM office_automation_task T 
            LEFT JOIN office_automation_task_receipt TR on T.id = TR.task_id 
            JOIN user U on u.id = TR.executor 
            JOIN topic_settings S on T.id = S.associate_task_id 
            WHERE consignor = $1;";
            sqlx::query_as(sql).bind(consignor).fetch_all(pool).await
        })
        .await?
        .map(Into::<QueryTasks>::into)
        .0.serde_to_string()
    }

    #[tauri::command]
    pub(crate) async fn get_task_list_by_typ(typ: String) -> Result<String, Error> {
        SqliteTask::query(async move |pool| {
            let sql = "SELECT T.id, hashkey, T.name, T.des task_des, typ, consignor, deadline, T.timestamp task_timestamp, 
            task_id, executor, u.name AS executor_name, status, TR.des receipt_des, TR.timestamp receipt_timestamp 
            FROM office_automation_task T 
            LEFT JOIN office_automation_task_receipt TR on T.id = TR.task_id 
            JOIN user U on u.id = TR.executor 
            JOIN topic_settings S on T.id = S.associate_task_id 
            WHERE typ = $1;";
            sqlx::query_as(sql).bind(typ).fetch_all(pool).await
        })
        .await?
        .map(Into::<QueryTasks>::into)
        .0.serde_to_string()
    }

    #[tauri::command]
    pub(crate) async fn get_task_info(task_id: &str) -> Result<String, Error> {
        let task_id = task_id.transform()?;
        SqliteTask::query(async move |pool| {
            let sql = "SELECT T.id, hashkey, T.name, T.des task_des, typ, consignor, deadline, T.timestamp task_timestamp, 
            task_id, executor, u.name AS executor_name, status, TR.des receipt_des, TR.timestamp receipt_timestamp 
            FROM office_automation_task T 
            LEFT JOIN office_automation_task_receipt TR on T.id = TR.task_id 
            JOIN user U on u.id = TR.executor 
            JOIN topic_settings S on T.id = S.associate_task_id 
            WHERE T.id = $1;";
            sqlx::query_as(sql).bind(task_id).fetch_all(pool).await
        })
        .await?
        .map(Into::<QueryTasks>::into)
        .0.serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_assign_task() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(15959119437, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        assign_task(
            Some("7030073914839805946"),
            Some("单元测试stream"),
            None,
            "单元测试topic",
            "单元测试任务".to_string(),
            Some("测试".to_string()),
            "group".to_string(),
            "1606174953750073345",
            "999999999",
        )
        .await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "Login" => {
                            assert_eq!(event, "Login");
                            // return;
                        }
                        "AssignTask" => {
                            assert_eq!(event, "AssignTask");
                            // return;
                        }
                        "AddTaskToTopic" => {
                            assert_eq!(event, "AddTaskToTopic");
                            // return;
                        }
                        "SendTaskMessage" => {
                            assert_eq!(event, "SendTaskMessage");
                            // return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_task_list_by_consignor() {
        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://test.db")
            .await
            .unwrap();
        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let res =
            crate::command::office_automation_task::query::get_task_list_by_consignor("123123")
                .await;
        println!("res: {res:#?}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_task_list() {
        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://15959119437.db")
            .await
            .unwrap();
        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let res =
            crate::command::office_automation_task::query::get_task_list("7039165602011033594")
                .await
                .unwrap();
        let res: std::collections::BTreeMap<i64, query::QueryTask> =
            serde_json::from_str(&res).unwrap();
        println!("res: {res:#?}");
    }
}
