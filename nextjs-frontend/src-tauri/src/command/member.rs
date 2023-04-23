use payload::resources::member::GroupMember;

use super::*;

#[tauri::command]
pub(crate) async fn add_member(uid: &str, gid: &str, level: i16) -> Result<(), Error> {
    let member = GroupMember::new(level);
    let member_action = GeneralAction::Insert {
        id: Some((uid.transform()?, gid.transform()?)),
        resource: member,
    };
    let member = Resources::Member(Command::new(
        gen_id().await,
        member_action,
        "AddMember".to_string(),
    ));
    let cmds = Commands::Single(member);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("add_member: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn dismiss_member(uid: &str, gid: &str) -> Result<(), Error> {
    let member_action = GeneralAction::Drop((uid.transform()?, gid.transform()?));
    let member = Resources::Member(Command::new(
        gen_id().await,
        member_action,
        "DismissMember".to_string(),
    ));
    let cmds = Commands::Single(member);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("dismiss_member: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn update_group_member_info(
    uid: &str,
    gid: &str,
    level: i16,
) -> Result<(), Error> {
    let member = GroupMember::new(level);

    let member_action = GeneralAction::Upsert {
        id: Some((uid.transform()?, gid.transform()?)),
        resource: member,
    };
    let group = Resources::Member(Command::new(
        gen_id().await,
        member_action,
        "UpdateMemberInfo".to_string(),
    ));

    let cmds = Commands::Single(group);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_group_member_info: {text}");
    send(TungMessage::Text(text)).await
}

pub mod query {
    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct PrivateMember {
        sender: i64,
        receiver: i64,
    }

    type Member = (String);
    type Members = (std::collections::HashSet<Member>);

    #[tauri::command]
    pub(crate) async fn get_private_chat_list(uid: &str) -> Result<String, Error> {
        tracing::info!("get_private_chat_list uid: {uid}");
        PrivateMember::query(async move |pool| {
            let sql = "SELECT sender, receiver FROM message 
        WHERE addr_typ = 'private' GROUP BY sender, receiver;";
            sqlx::query_as(sql).bind(uid).fetch_all(pool).await
        })
        .await?
        .map(|res| {
            tracing::info!("get private chat list res: {res:?}");
            res.iter()
                .map(|m| {
                    if m.receiver.to_string() == uid {
                        m.sender.to_string()
                    } else {
                        m.receiver.to_string()
                    }
                })
                .collect::<Vec<Member>>()
        })
        .serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_add_member() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        add_member("1001", "7030073914839805946", 7).await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "AddMember" => {
                            assert_eq!(event, "AddMember");
                            return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_dismiss_member() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        dismiss_member("1001", "7030073914839805946").await;
        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "DismissMember" => {
                            assert_eq!(event, "DismissMember");
                            return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_group_member_info() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        update_group_member_info("1001", "7030073914839805946", 5).await;
        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    assert_eq!(event, "UpdateMemberInfo");
                }
            }
        }
    }
}
