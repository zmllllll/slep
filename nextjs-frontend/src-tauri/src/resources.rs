use super::*;
use anyhow::Result;
use payload::resources::{
    group::Group, member::GroupMember, message::Message, office_automation_task::OATask,
    profile_picture::ProfilePicture, read_status::ReadStatus, stream::StreamSettings,
    stream_level::StreamLevel, task_id::TaskId, task_receipt::TaskReceipt, topic::TopicSettings,
    topic_level::TopicLevel, user::User, username::Username,
};
use resource::{Command, GeneralAction};
use sqlx::{database::Database as SqlxDatabase, Any, Executor as SqlxExecutor, Postgres, Sqlite};

#[derive(Deserialize, Serialize, Debug)]
pub enum Resources {
    User(Command<GeneralAction<Sqlite, User>>),
    Username(Command<GeneralAction<Sqlite, Username>>),
    ProfilePicture(Command<GeneralAction<Sqlite, ProfilePicture>>),
    Message(Command<GeneralAction<Sqlite, Message>>),
    ReadStatus(Command<GeneralAction<Sqlite, ReadStatus>>),
    Group(Command<GeneralAction<Sqlite, Group>>),
    Member(Command<GeneralAction<Sqlite, GroupMember>>),
    Reviewer(Command<GeneralAction<Sqlite, GroupMember>>),
    StreamSettings(Command<GeneralAction<Sqlite, StreamSettings>>),
    StreamLevel(Command<GeneralAction<Sqlite, StreamLevel>>),
    TopicSettings(Command<GeneralAction<Sqlite, TopicSettings>>),
    TopicLevel(Command<GeneralAction<Sqlite, TopicLevel>>),
    Task(Command<GeneralAction<Sqlite, OATask>>),
    TaskId(Command<GeneralAction<Sqlite, TaskId>>),
    TaskReceipt(Command<GeneralAction<Sqlite, TaskReceipt>>),
}

impl resource::Action for Resources {
    async fn execute<'e, 'c: 'e, E>(&'e self, executor: E) -> Result<()>
    where
        E: SqlxExecutor<'c, Database = Any>,
    {
        match self {
            Resources::User(r) => r.execute(executor).await,
            Resources::Username(r) => r.execute(executor).await,
            Resources::ProfilePicture(r) => r.execute(executor).await,
            Resources::Message(r) => r.execute(executor).await,
            Resources::ReadStatus(r) => r.execute(executor).await,
            Resources::Group(r) => r.execute(executor).await,
            Resources::Member(r) => {
                let uid = command::user::SELF
                    .read()
                    .await
                    .0
                    .ok_or(error::Error::System("User ID Not Exist".to_string()))?;

                let Command { action, .. } = r;
                if let Some(pkey) = match action {
                    GeneralAction::Insert { id, .. } | GeneralAction::Upsert { id, .. } => {
                        id.as_ref()
                    }
                    GeneralAction::Update { id, .. } | GeneralAction::Drop(id) => Some(id),
                } {
                    let (member_id, _gid) = pkey;
                    if member_id == &uid {
                        init::pulling(uid).await
                    } else {
                        r.execute(executor).await
                    }
                } else {
                    r.execute(executor).await
                }
            }
            Resources::Reviewer(r) => r.execute(executor).await,
            Resources::StreamSettings(r) => r.execute(executor).await,
            Resources::StreamLevel(r) => r.execute(executor).await,
            Resources::TopicSettings(r) => r.execute(executor).await,
            Resources::TopicLevel(r) => r.execute(executor).await,
            Resources::Task(r) => r.execute(executor).await,
            Resources::TaskId(r) => r.execute(executor).await,
            Resources::TaskReceipt(r) => r.execute(executor).await,
        }
    }
}

impl resource::Resources for Resources {}
