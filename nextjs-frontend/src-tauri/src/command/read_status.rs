use payload::resources::read_status::{Addr, AddrType, ReadStatus};

use super::*;

#[tauri::command]
pub(crate) async fn read(read_status: ReadStatus) -> Result<(), Error> {
    let action = GeneralAction::Upsert {
        id: None,
        resource: read_status,
    };
    let read_status = Resources::ReadStatus(Command::new(
        gen_id().await,
        action,
        "UpdateReadStatus".to_string(),
    ));
    let cmds = Commands::Single(read_status);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_read_status: {text}");
    send(TungMessage::Text(text)).await
}

pub mod query {
    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow)]
    pub(crate) struct ReadStatusTemp {
        addr: String,
        latest_message_id: i64,
    }

    impl TryFrom<ReadStatusTemp> for ReadStatus {
        type Error = Error;

        fn try_from(value: ReadStatusTemp) -> std::result::Result<Self, Error> {
            let addr: Addr = serde_json::from_str(&value.addr)
                .map_err(|e| Error::System(format!("serde addr error: {e}")))?;
            Ok(Self {
                addr,
                latest_message_id: value.latest_message_id,
            })
        }
    }

    #[derive(Debug, Serialize, sqlx::FromRow)]
    struct CountTemp {
        pub(self) count: i64,
    }

    impl CountTemp {
        fn count(&self) -> u32 {
            let count = self.count;
            if count <= 999 {
                count as u32
            } else {
                999
            }
        }
    }

    use std::collections::HashMap;
    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct ReadStatusWithNumber {
        groups: HashMap<i64, GroupStatus>,
        privates: HashMap<i64, TopicsStatus>,
        count: u32,
    }

    impl ReadStatusWithNumber {
        async fn from_read_status_vec(status_vec: &[ReadStatus]) -> Result<Self, Error> {
            // gid, stream, topic, latest_message_id, count
            let mut groups: HashMap<i64, GroupStatus> = HashMap::new();
            // uid, topic, latest_message_id, count
            let privates: HashMap<i64, TopicsStatus> = HashMap::new();
            for status in status_vec {
                let ReadStatus {
                    addr:
                        Addr {
                            uid,
                            addr_type,
                            topic,
                        },
                    latest_message_id,
                } = status;

                match addr_type {
                    // FIXME:
                    AddrType::Private { receiver } => continue,
                    AddrType::Stream { gid, stream } => {
                        let count =
                            Self::count_stream(*gid, stream, topic, *latest_message_id).await?;
                        groups
                            .entry(*gid)
                            .and_modify(|group| {
                                group.insert(stream, topic, *latest_message_id, count)
                            })
                            .or_insert_with(|| {
                                (stream.as_str(), topic.as_str(), *latest_message_id, count).into()
                            });
                    }
                }
            }

            let count = groups.values().fold(0, |acc, group| acc + group.count);
            Ok(Self {
                groups,
                privates,
                count,
            })
        }

        async fn count_stream(
            gid: i64,
            stream: &str,
            topic: &str,
            latest_message_id: i64,
        ) -> Result<u32, Error> {
            if let Some(pool) = unsafe { crate::SQLITE_POOL.get() } {
                let sql =
                    "SELECT count(id) as count FROM message WHERE gid = $1 AND addr = $2 AND topic = $3 where id > latest_message_id;";
                let count: CountTemp = sqlx::query_as(sql)
                    .bind(gid)
                    .bind(stream)
                    .bind(topic)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::DataQueryFailed("sqlite".to_owned(), e.to_string()))?;
                let count = count.count();
                Ok(count)
            } else {
                Err(Error::PoolGetFailed("sqlite".to_owned()))
            }
        }

        // FIXME:
        async fn count_private(uid: i64, topic: &str) -> Result<u32, Error> {
            if let Some(pool) = unsafe { crate::SQLITE_POOL.get() } {
                let sql =
                    "SELECT count(id) as count FROM message WHERE gid = NULL AND (addr = $1 OR sender = $1) AND topic = $2";
                let count: CountTemp = sqlx::query_as(sql)
                    .bind(uid)
                    .bind(topic)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::DataQueryFailed("sqlite".to_owned(), e.to_string()))?;
                let count = count.count();
                Ok(count)
            } else {
                Err(Error::PoolGetFailed("sqlite".to_owned()))
            }
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub(crate) struct GroupStatus {
        // Stream: count
        streams: HashMap<String, TopicsStatus>,
        count: u32,
    }

    impl GroupStatus {
        fn insert(&mut self, stream: &str, topic: &str, latest_message_id: i64, count: u32) {
            if self
                .streams
                .insert(stream.to_string(), (topic, latest_message_id, count).into())
                .is_none()
            {
                self.count += count;
            }
        }
    }

    impl From<(&str, &str, i64, u32)> for GroupStatus {
        fn from(value: (&str, &str, i64, u32)) -> Self {
            let (stream, topic, latest_message_id, count) = value;
            let streams =
                HashMap::from([(stream.to_string(), (topic, latest_message_id, count).into())]);
            Self { streams, count }
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub(crate) struct TopicsStatus {
        // Topic: count
        topics: HashMap<String, TopicStatus>,
        count: u32,
    }

    impl TopicsStatus {
        fn insert(&mut self, topic: &str, latest_message_id: i64, count: u32) {
            if self
                .topics
                .insert(topic.to_string(), (latest_message_id, count).into())
                .is_none()
            {
                self.count += count;
            }
        }
    }

    // topic latest_message_id, count
    impl From<(&str, i64, u32)> for TopicsStatus {
        fn from(value: (&str, i64, u32)) -> Self {
            let (topic, latest_message_id, count) = value;
            let topics = HashMap::from([(topic.to_string(), (latest_message_id, count).into())]);
            Self { topics, count }
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub(crate) struct TopicStatus {
        latest_message_id: i64,
        count: u32,
    }

    impl From<(i64, u32)> for TopicStatus {
        fn from(value: (i64, u32)) -> Self {
            let (latest_message_id, count) = value;
            Self {
                latest_message_id,
                count,
            }
        }
    }

    #[tauri::command]
    pub(crate) async fn get_read_status() -> Result<String, Error> {
        let uid = user::SELF
            .read()
            .await
            .0
            .ok_or(Error::System("Self User ID Not Exist".to_string()))?;

        let status_vec = ReadStatusTemp::query(async move |pool| {
            let sql = "SELECT addr, latest_message_id FROM read_status;";
            sqlx::query_as(sql).fetch_all(pool).await
        })
        .await?
        .into_iter()
        .map(TryInto::<ReadStatus>::try_into)
        .try_collect::<Vec<ReadStatus>>()?
        .into_iter()
        .filter(|status| status.addr.uid == uid)
        .collect::<Vec<ReadStatus>>();

        ReadStatusWithNumber::from_read_status_vec(&status_vec)
            .await?
            .serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use resource::Resource;
    use sqlx::migrate::MigrateDatabase;

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_read_status() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        crate::STORAGE.set(std::path::PathBuf::from(r"./")).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1, "local_test").await;
            println!("res: {res:?}");
        });

        let s1 = ReadStatus {
            addr: Addr {
                uid: 1,
                addr_type: AddrType::Stream {
                    gid: 1,
                    stream: "test stream".to_string(),
                },
                topic: "test topic".to_string(),
            },
            latest_message_id: 1,
        };

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        read(s1).await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "UpdateReadStatus" => {
                            assert_eq!(event, "UpdateReadStatus");
                            return;
                        }
                        "Initialize" => continue,
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_read() {
        let mut write = user::SELF.write().await;
        write.0 = Some(1);
        drop(write);
        let s1 = ReadStatus {
            addr: Addr {
                uid: 1,
                addr_type: AddrType::Stream {
                    gid: 1,
                    stream: "test stream".to_string(),
                },
                topic: "test topic".to_string(),
            },
            latest_message_id: 1,
        };

        let s2 = ReadStatus {
            addr: Addr {
                uid: 1,
                addr_type: AddrType::Stream {
                    gid: 2,
                    stream: "test stream".to_string(),
                },
                topic: "test topic".to_string(),
            },
            latest_message_id: 2,
        };

        let s3 = ReadStatus {
            addr: Addr {
                uid: 1,
                addr_type: AddrType::Stream {
                    gid: 3,
                    stream: "test stream".to_string(),
                },
                topic: "test topic".to_string(),
            },
            latest_message_id: 3,
        };

        let s4 = ReadStatus {
            addr: Addr {
                uid: 2,
                addr_type: AddrType::Stream {
                    gid: 4,
                    stream: "test stream".to_string(),
                },
                topic: "test topic".to_string(),
            },
            latest_message_id: 4,
        };

        sqlx::Sqlite::drop_database("sqlite://test.db").await;
        sqlx::Sqlite::create_database("sqlite://test.db").await;

        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect("sqlite://test.db")
            .await
            .unwrap();
        // sqlx::sqlite::SqlitePoolOptions::new()
        sqlx::migrate!("./migrations")
            .run(&sqlite_pool)
            .await
            .unwrap();

        <ReadStatus as Resource<sqlx::Sqlite>>::upsert(&s1, &None, &sqlite_pool)
            .await
            .unwrap();
        <ReadStatus as Resource<sqlx::Sqlite>>::upsert(&s2, &None, &sqlite_pool)
            .await
            .unwrap();
        <ReadStatus as Resource<sqlx::Sqlite>>::upsert(&s3, &None, &sqlite_pool)
            .await
            .unwrap();
        <ReadStatus as Resource<sqlx::Sqlite>>::upsert(&s4, &None, &sqlite_pool)
            .await
            .unwrap();

        unsafe {
            let _ = crate::SQLITE_POOL.set(sqlite_pool);
        }
        let status = query::get_read_status().await.unwrap();
        println!("get_read_status: {:#?}", status);
        let res = "[{\"addr\":{\"uid\":1,\"gid\":1,\"stream\":\"test stream\",\"topic\":\"test topic\"},\"latest_message_id\":1},{\"addr\":{\"uid\":1,\"gid\":2,\"stream\":\"test stream\",\"topic\":\"test topic\"},\"latest_message_id\":2},{\"addr\":{\"uid\":1,\"gid\":3,\"stream\":\"test stream\",\"topic\":\"test topic\"},\"latest_message_id\":3}]".to_string();
        assert_eq!(res, status)
    }
}
