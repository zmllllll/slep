use super::*;
use payload::resources::{group::Group, member::GroupMember};

#[tauri::command]
pub(crate) async fn create_group(
    uid: &str,
    pid: Option<&str>,
    group_name: String,
    des: Option<String>,
) -> Result<(), Error> {
    let group = Group::new(pid.transform()?, group_name, des);

    let gid = payload::resources::gen_id().await;
    let group_action = GeneralAction::Upsert {
        id: Some(gid),
        resource: group,
    };

    let group = Resources::Group(Command::new(
        gen_id().await,
        group_action,
        "CreateGroup".to_string(),
    ));

    let member = GroupMember::new(1);
    let member_action = GeneralAction::Upsert {
        id: Some((uid.transform()?, gid)),
        resource: member,
    };
    let member = Resources::Member(Command::new(
        gen_id().await,
        member_action,
        "AddMember".to_string(),
    ));
    let cmds = Commands::Multi(vec![group, member]);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("create_group: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn dismiss_group(uid: &str, gid: &str) -> Result<(), Error> {
    let mut resources = Vec::new();

    let gid = gid.transform()?;
    let group_action = GeneralAction::Drop(gid);
    let group = Resources::Group(Command::new(
        gen_id().await,
        group_action,
        "DismissGroup".to_string(),
    ));
    resources.push(group);

    let query: Vec<query::GroupMember> = query::sqlite_member_query(gid).await?;
    for member in query {
        let member_action = GeneralAction::Drop((member.uid, gid));
        let member = Resources::Member(Command::new(
            gen_id().await,
            member_action,
            "DismissMember".to_string(),
        ));
        resources.push(member);
    }

    let cmds = Commands::Multi(resources);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("dismiss_group: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn update_group_info(
    gid: &str,
    pid: Option<&str>,
    group_name: String,
    des: Option<String>,
) -> Result<(), Error> {
    let group = Group::new(pid.transform()?, group_name, des);

    let group_action = GeneralAction::Upsert {
        id: Some(gid.transform()?),
        resource: group,
    };
    let group = Resources::Group(Command::new(
        gen_id().await,
        group_action,
        "UpdateGroupInfo".to_string(),
    ));

    let cmds = Commands::Single(group);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_group_info: {text}");
    send(TungMessage::Text(text)).await
}

pub mod query {
    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct Group {
        gid: i64,
        name: String,
        pid: Option<i64>,
        des: String,
        level: i16,
    }

    #[tauri::command]
    pub(crate) async fn get_group_by_uid(uid: &str) -> Result<String, Error> {
        let uid = uid.transform()?;
        Group::query(async move |pool| {
            let sql = "SELECT gid, name, pid, des, level FROM group_member 
            JOIN user_group ON id = gid 
            WHERE uid =$1;";
            sqlx::query_as(sql).bind(uid).fetch_all(pool).await
        })
        .await?
        .serde_to_string()
    }

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct GroupMember {
        gid: i64,
        pub uid: i64,
        level: i16,
    }

    pub(crate) async fn sqlite_member_query(gid: i64) -> Result<Vec<GroupMember>, Error> {
        GroupMember::query(async move |pool| {
            let sql = "SELECT gid, uid, level FROM group_member WHERE gid =$1;";
            sqlx::query_as(sql).bind(gid).fetch_all(pool).await
        })
        .await
    }

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct SqliteGroupMember {
        gid: i64,
        pub uid: i64,
        name: String,
        level: i16,
    }

    #[tauri::command]
    pub(crate) async fn get_group_member_list(gid: &str) -> Result<String, Error> {
        let gid = gid.transform()?;
        SqliteGroupMember::query(async move |pool| {
            let sql = "SELECT gid, uid, name, level FROM group_member 
            JOIN user ON id = uid
            WHERE gid =$1;";
            sqlx::query_as(sql).bind(gid).fetch_all(pool).await
        })
        .await?
        .serde_to_string()
    }

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct SubGroup {
        id: i64,
        pid: i64,
        name: String,
        des: String,
    }

    #[tauri::command]
    pub(crate) async fn get_sub_group(pid: &str) -> Result<String, Error> {
        let pid = pid.transform()?;
        SubGroup::query(async move |pool| {
            let sql = "SELECT id, pid, name, des FROM user_group WHERE pid = $1;";
            sqlx::query_as(sql).bind(pid).fetch_all(pool).await
        })
        .await?
        .serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_group() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        create_group(
            "1606174953750073345",
            None,
            "qwj_555".to_string(),
            Some("test".to_string()),
        )
        .await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "CreateGroup" => assert_eq!(event, "CreateGroup"),
                        "AddMember" => assert_eq!(event, "AddMember"),
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_dismiss_group() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        dismiss_group("1606174953750073345", "7030089276159438842").await;
        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "DismissGroup" => assert_eq!(event, "DismissGroup"),
                        "DismissMember" => assert_eq!(event, "DismissMember"),
                        _ => panic!(),
                    }
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_update_group_info() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        update_group_info("7030073914839805946", None, "单元测试组".to_string(), None).await;
        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    assert_eq!(event, "UpdateGroupInfo");
                }
            }
        }
    }
}
