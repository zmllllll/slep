use std::borrow::Cow;

use payload::resources::{profile_picture::ProfilePicture, user::User, username::Username};
use tauri::async_runtime::RwLock;
use tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

use crate::CLIENT;

use super::*;

// pub(crate) static SELF: once_cell::sync::OnceCell<i64> = once_cell::sync::OnceCell::new();
pub(crate) static SELF: once_cell::sync::Lazy<RwLock<Uid>> =
    once_cell::sync::Lazy::new(|| RwLock::new(Uid(None)));

pub(crate) struct Uid(pub(crate) Option<i64>);

#[tauri::command]
pub(crate) async fn login(uid: &str) -> Result<(), Error> {
    let uid = uid.transform()?;
    let mut write = SELF.write().await;
    write.0 = Some(uid);
    drop(write);
    if let Err(e) = crate::websocket::connect(uid, "test").await {
        tracing::info!("websocket connect error: {e:?}");
        return Err(Error::UserNotExist(e.to_string()));
    };
    Ok(())
}

#[tauri::command]
pub(crate) async fn log_out() -> Result<(), Error> {
    let mut client = CLIENT.lock().await;
    client.log_status = true;
    send(TungMessage::Close(Some(CloseFrame {
        code: CloseCode::Normal,
        reason: Cow::Owned("log out".to_string()),
    })))
    .await
}

#[tauri::command]
pub(crate) async fn rename_username(uid: &str, name: String) -> Result<(), Error> {
    let user = Username::new(name);
    let user_action = GeneralAction::Upsert {
        id: Some(uid.transform()?),
        resource: user,
    };
    let user = Resources::Username(Command::new(
        gen_id().await,
        user_action,
        "RenameUsername".to_string(),
    ));
    let cmds = Commands::Single(user);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("rename_username: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn update_profile_picture(
    uid: &str,
    profile_picture: String,
) -> Result<(), Error> {
    let user = ProfilePicture::new(Some(profile_picture));
    let user_action = GeneralAction::Upsert {
        id: Some(uid.transform()?),
        resource: user,
    };
    let user = Resources::ProfilePicture(Command::new(
        gen_id().await,
        user_action,
        "UpdateProfilePicture".to_string(),
    ));
    let cmds = Commands::Single(user);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("update_profile_picture: {text}");
    send(TungMessage::Text(text)).await
}

// pub(crate) async fn complement_user_info(uid: i64) -> Result<(), Error> {
//     use random_string::generate;
//     let name = generate(15, "abcdefghijklmnopqrstuvwxyz0123456789_");
//     let user = User::new(name);
//     let user_action = GeneralAction::Insert {
//         id: Some(uid),
//         resource: user,
//     };
//     let user = Resources::User(Command::new(
//         gen_id().await,
//         user_action,
//         "ComplementUserInfo".to_string(),
//     ));
//     let cmds = Commands::Single(user);
//     let text = serde_json::to_string(&cmds).unwrap();
//     tracing::info!("complement_user_info: {text}");
//     send(TungMessage::Text(text)).await
// }

pub mod query {
    use super::*;

    #[derive(Debug, Serialize, sqlx::FromRow, Default)]
    pub(crate) struct UserInfo {
        name: String,
    }

    #[tauri::command]
    pub(crate) async fn get_user_info(uid: &str) -> Result<String, Error> {
        let uid = uid.transform()?;
        UserInfo::query(async move |pool| {
            let sql = "SELECT name FROM user WHERE id = $1;";
            sqlx::query_as(sql).bind(uid).fetch_all(pool).await
        })
        .await?
        .serde_to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_rename_username() {
        let (notify_tx, notify_rx) =
            tokio::sync::mpsc::unbounded_channel::<crate::notify::Notify>();
        let mut notify_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(notify_rx);
        crate::NOTIFY_TX.set(notify_tx).unwrap();
        tokio::task::spawn(async {
            let res = crate::websocket::connect(1606174953750073345, "qwj").await;
            println!("res: {res:?}");
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        rename_username("1606174953750073345", "牛逼".to_string()).await;

        use tokio_stream::StreamExt as _;
        while let Some(notify) = notify_rx.next().await {
            match notify {
                crate::notify::Notify::SendResponse(ref event, res) => {
                    println!("res: event: {event}, res: {res:?}");
                    match event.as_str() {
                        "RenameUsername" => {
                            assert_eq!(event, "RenameUsername");
                            return;
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }
}
