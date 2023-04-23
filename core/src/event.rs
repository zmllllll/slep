use crate::{Sender, TokioOneShotSender};
use std::{collections::HashMap, sync::Mutex};

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum Event {
    // Connect: uid, UUID, sender
    Connect(i64, String, Sender),
    // Disconnect: uid, UUID
    Disconnect(i64, String),

    // ACKTask: uid, uuid, trace
    ACKTask(i64, String, i64),
    // TimeoutTask: uid, uuid, trace_id
    // TimeoutTask(i64, String, i64),
    // Ping
    Ping,
    // PongTask: uid, msg
    Pong(i64, Vec<u8>),
    // Resource: uid, Commands
    Resource(
        i64,
        Box<resource::Commands<crate::resources::Resources>>,
        Option<TokioOneShotSender<Result<(), anyhow::Error>>>,
    ),
}

pub(super) enum ConnectionEvent {
    Message(String),
    #[allow(dead_code)]
    Ping(Vec<u8>),
    Disconnect,
}

// pub(crate) static STORAGE: once_cell::sync::Lazy<Mutex<crate::storage::StorageManager>> =
//     once_cell::sync::Lazy::new(|| Mutex::new(crate::storage::StorageManager::default()));

pub(super) async fn handle(mut collect_rx: tokio_stream::wrappers::UnboundedReceiverStream<Event>) {
    // let pool =
    //     sqlx::Pool::<sqlx::Any>::connect("postgres://postgres:great1996@localhost:5432/slep")
    let pool = sqlx::Pool::<sqlx::Any>::connect(
        // "postgresql://1.15.14.20?dbname=slep&user=postgres&password=quake@123",
        "postgres://postgres:123456@localhost:5432/slep",
    )
    .await
    .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let mut users: HashMap<i64, Sender> = HashMap::new();
    let mut storage_manager = crate::storage::StorageManager::default().init(&pool).await;
    // {
    //     let mut storage = STORAGE.lock().unwrap();
    //     storage.init(&pool).await;
    // }

    let _ = crate::PG_POOL.set(pool);
    use tokio_stream::StreamExt as _;
    while let Some(event) = collect_rx.next().await {
        match event {
            Event::Connect(username, ref _uuid, sender) => {
                users.entry(username).or_insert_with(|| sender);
            }
            Event::Disconnect(username, _uuid) => {
                if let Some(sender) = users.remove(&username) {
                    drop(sender);
                }
                tracing::info!("devices disconnect: {:#?}", users);
            }
            Event::ACKTask(user, uuid, trace) => {
                tracing::info!("`[{user}]`[{uuid}] get right ACK: {trace}");
            }
            // Event::TimeoutTask(user, uuid, trace) => {
            //     if let Some(conns) = users.get_mut(&user) {};
            // }
            Event::Ping => users.retain(|_username, _conns| todo!()),
            Event::Pong(_user, _msg) => {
                // use sessions::models::EasyRlp as _;
                // let p = sessions::models::Ping::from_rlp(&msg);
                // if let Some(conns) = users.get_mut(&user) {
                //     if let Some(conn) = conns.get_mut(&p.uuid) {
                //         conn.get_pong(p.heartbeat);
                //         // tracing::info!(
                //         //     "get [{}]{} pong, delay time: {:?}ms",
                //         //     user,
                //         //     p.uuid,
                //         //     conn.delay()
                //         // );
                //     };
                // };
            }
            Event::Resource(uid, cmds, sender) => {
                tracing::info!("[Resource] get cmd: {:?}", cmds);
                if let Err(e) = cmds.execute(crate::PG_POOL.get().unwrap()).await {
                    tracing::error!("[Resource] command execute error: {:?}", e);
                    if let Some(sender) = sender {
                        let _ = sender.send(Err(e));
                    }
                } else {
                    if let Some(sender) = sender {
                        let _ = sender.send(Ok(()));
                        continue;
                    }
                    use crate::builder::Generator as _;
                    let mut updater = cmds.generate(uid);
                    updater.update_all(&mut storage_manager);
                    tracing::info!("ready to filter receiver\nreport: {cmds:?}");

                    use crate::builder::Consumer as _;
                    // let storage_manager = STORAGE.lock().unwrap();
                    match cmds.consume((&storage_manager, uid)) {
                        Ok(dispatcher) => {
                            tracing::info!("dispatcher: {dispatcher:?}");
                            dispatcher.dispatch_all(&mut users);
                            updater.leave(&mut storage_manager);
                        }
                        Err(e) => {
                            tracing::error!("[Resource] resources dispatch error: {e}");
                        }
                    }
                };
            }
        }
    }
}
