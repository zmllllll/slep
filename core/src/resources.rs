use super::{Deserialize, Serialize};
use anyhow::Result;
use payload::resources::{
    group::Group, member::GroupMember, message::Message, office_automation_task::OATask,
    profile_picture::ProfilePicture, read_status::ReadStatus, reviewer::Reviewer,
    stream::StreamSettings, stream_level::StreamLevel, task_id::TaskId, task_receipt::TaskReceipt,
    topic::TopicSettings, topic_level::TopicLevel, user::User, username::Username,
};
use resource::{Command, Commands, GeneralAction};
use sqlx::{Any, Executor as SqlxExecutor, Postgres};

use std::collections::HashSet;

use crate::{
    builder::Generator,
    check,
    collect,
    // event::STORAGE,
    receiver::Receiver,
    storage::{
        action::{GroupAction, TopicAction},
        updater::Updater,
        StorageManager,
    },
};

mod group;
mod member;
mod message;
mod office_automation_task;
mod profile_picture;
mod read_status;
mod reviewer;
mod stream_level;
mod stream_settings;
mod task_id;
mod task_receipt;
mod topic_level;
mod topic_settings;
mod user;
mod username;
pub(crate) type Extension<'a> = (&'a StorageManager, i64);

#[derive(Deserialize, Serialize, Debug)]
pub enum Resources {
    User(Command<GeneralAction<Postgres, User>>),
    Username(Command<GeneralAction<Postgres, Username>>),
    ProfilePhoto(Command<GeneralAction<Postgres, ProfilePicture>>),
    Message(Command<GeneralAction<Postgres, Message>>),
    ReadStatus(Command<GeneralAction<Postgres, ReadStatus>>),
    Group(Command<GeneralAction<Postgres, Group>>),
    Member(Command<GeneralAction<Postgres, GroupMember>>),
    Reviewer(Command<GeneralAction<Postgres, Reviewer>>),
    StreamSettings(Command<GeneralAction<Postgres, StreamSettings>>),
    StreamLevel(Command<GeneralAction<Postgres, StreamLevel>>),
    TopicSettings(Command<GeneralAction<Postgres, TopicSettings>>),
    TopicLevel(Command<GeneralAction<Postgres, TopicLevel>>),
    Task(Command<GeneralAction<Postgres, OATask>>),
    TaskId(Command<GeneralAction<Postgres, TaskId>>),
    TaskReceipt(Command<GeneralAction<Postgres, TaskReceipt>>),
}

impl resource::Action for Resources {
    async fn execute<'e, 'c: 'e, E>(&'e self, executor: E) -> Result<()>
    where
        E: SqlxExecutor<'c, Database = Any>,
    {
        match self {
            Resources::User(r) => r.execute(executor).await,
            Resources::Username(r) => r.execute(executor).await,
            Resources::ProfilePhoto(r) => r.execute(executor).await,
            Resources::Message(r) => {
                // let storage_manager = STORAGE.lock().unwrap();
                // match &r.action {
                //     GeneralAction::Insert { id: _, resource }
                //     | GeneralAction::Upsert { id: _, resource }
                //     | GeneralAction::Update { id: _, resource } => {
                //         use check::Check as _;
                //         //TODO: 加私聊
                //         if let Some(gid) = resource.gid
                //         && let Some(group) = storage_manager.groups.get(&gid)
                //         && let Some(s) = group.get_stream().get(&resource.addr)
                //         && let Some(sender) = group.get_members().get(&resource.sender)
                //         && sender.check_level(check::Constraint::Range(check::Compare::Le(
                //             check::UpperLimit(s.write_level),
                //         ))){
                //             r.execute(executor).await
                //         }else{
                //             //TODO:
                //             // self
                //             // Err()
                //             Ok(())
                //         }
                //     }
                //     GeneralAction::Drop(_) => r.execute(executor).await,
                // }
                r.execute(executor).await
            }
            Resources::ReadStatus(r) => r.execute(executor).await,
            Resources::Group(r) => r.execute(executor).await,
            Resources::Member(r) => r.execute(executor).await,
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
