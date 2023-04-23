use payload::resources::member::GroupMember;

use super::*;

#[tauri::command]
pub(crate) async fn appoint_reviewer(uid: &str, gid: &str) -> Result<(), Error> {
    let member = GroupMember::new(crate::constant::REVIEWER);
    let member_action = GeneralAction::Update {
        id: (uid.transform()?, gid.transform()?),
        resource: member,
    };
    let member = Resources::Reviewer(Command::new(
        gen_id().await,
        member_action,
        "AppointReviewer".to_string(),
    ));

    let cmds = Commands::Single(member);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("appoint_reviewer: {text}");
    send(TungMessage::Text(text)).await
}

#[tauri::command]
pub(crate) async fn dismiss_reviewer(uid: &str, gid: &str) -> Result<(), Error> {
    let member = GroupMember::new(crate::constant::MEMBER);
    let member_action = GeneralAction::Update {
        id: (uid.transform()?, gid.transform()?),
        resource: member,
    };
    let member = Resources::Reviewer(Command::new(
        gen_id().await,
        member_action,
        "DismissReviewer".to_string(),
    ));
    let cmds = Commands::Single(member);
    let text = serde_json::to_string(&cmds).unwrap();
    tracing::info!("dismiss_reviewer: {text}");
    send(TungMessage::Text(text)).await
}
