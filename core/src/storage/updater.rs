use std::sync::MutexGuard;

use resource::Commands;

use super::StorageManager;
use crate::{builder::Generator, resources::Resources};

#[macro_export]
macro_rules! gen_updater {
    ($r: expr, $uid: expr, $($items: ident),*) => {{
        match $r {
            $(
                Resources::$items(cmd) => cmd.generate($uid),
            )*
            _ => None,
        }
    }};
}

// TODO: 套娃macro_rules

impl<'a> Generator<'a, Updaters> for Commands<Resources> {
    /// generate
    ///
    /// generate Updaters to update the storage data
    fn generate(&self, uid: Self::Ext) -> Updaters {
        match self {
            Commands::Single(cmd) => Updaters::Single(gen_updater!(
                cmd,
                uid,
                User,
                Message,
                Group,
                Member,
                Reviewer,
                StreamSettings,
                StreamLevel,
                TopicSettings,
                TopicLevel,
                TaskId
            )),
            Commands::Multi(cmds) => {
                let mut updaters = Vec::new();
                for cmd in cmds {
                    updaters.push(gen_updater!(
                        cmd,
                        uid,
                        User,
                        Message,
                        Group,
                        Member,
                        Reviewer,
                        StreamSettings,
                        StreamLevel,
                        TopicSettings,
                        TopicLevel,
                        TaskId
                    ))
                }
                Updaters::Multi(updaters)
            }
        }
    }

    type Ext = i64;
}

pub(crate) enum Updaters {
    Single(Option<Updater>),
    Multi(Vec<Option<Updater>>),
}

impl Updaters {
    pub(crate) fn update_all(&mut self, storage_manager: &mut StorageManager) {
        match self {
            Updaters::Single(op) => {
                if let Some(o) = op {
                    o.update(storage_manager)
                }
            }
            Updaters::Multi(ops) => {
                for op in ops.iter_mut().flatten() {
                    op.update(storage_manager)
                }
            }
        }
    }

    pub(crate) fn leave(&mut self, storage_manager: &mut StorageManager) {
        match self {
            Updaters::Single(op) => {
                if let Some(o) = op {
                    o.leave(storage_manager)
                }
            }
            Updaters::Multi(ops) => {
                for op in ops.iter_mut().flatten() {
                    op.leave(storage_manager)
                }
            }
        }
    }
}

pub(crate) enum Updater {
    /// 更新groups: gid, GroupAction
    Groups(i64, Vec<super::action::GroupAction>),
    /// 更新topics: hash("Gid+Stream+Topic"), TopicAction
    Topics(i64, Vec<super::action::TopicAction>),
}

impl Updater {
    fn update(&mut self, storage_manager: &mut StorageManager) {
        match self {
            Updater::Groups(gid, group_action) => {
                group_action
                    .iter_mut()
                    .for_each(|ga| ga.update(*gid, &mut storage_manager.groups));
                tracing::info!("groups op: {:#?}", storage_manager.groups);
            }
            Updater::Topics(hashkey, topic_action) => {
                topic_action
                    .iter_mut()
                    .for_each(|ta| ta.update(*hashkey, &mut storage_manager.topics));
                tracing::info!("topics op: {:#?}", storage_manager.topics);
            }
        }
    }

    fn leave(&mut self, storage_manager: &mut StorageManager) {
        match self {
            Updater::Groups(gid, group_action) => {
                group_action
                    .iter_mut()
                    .for_each(|ga| ga.leave(*gid, &mut storage_manager.groups));
                tracing::info!("groups op: {:#?}", storage_manager.groups);
            }
            Updater::Topics(hashkey, topic_action) => {
                topic_action
                    .iter_mut()
                    .for_each(|ta| ta.leave(*hashkey, &mut storage_manager.topics));
                tracing::info!("topics op: {:#?}", storage_manager.topics);
            }
        }
    }
}
