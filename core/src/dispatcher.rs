use std::collections::HashMap;

use crate::{
    builder::{Consumer, Generator as _},
    event::ConnectionEvent,
    receiver::Receiver,
    resources::Resources,
};
use anyhow::Result;
use resource::Commands;

/// Dispatcher
///
/// send resources to clients
#[derive(Debug)]
pub enum Dispatcher {
    Single(Box<Commands<Resources>>, Receiver),
    Multi(Vec<(Box<Commands<Resources>>, Receiver)>),
}

impl Dispatcher {
    /// dispatch_all
    ///
    /// dispatch all of the resources to clients, clients are obtained after filtering
    pub(crate) fn dispatch_all(
        self,
        users: &mut HashMap<i64, tokio::sync::mpsc::UnboundedSender<ConnectionEvent>>,
    ) {
        match self {
            Dispatcher::Single(cmds, recv) => {
                recv.dispatch(&cmds, users);
            }
            Dispatcher::Multi(v) => {
                for (cmds, recv) in v {
                    tracing::info!("cmds: {cmds:?},\nreceiver:{recv:?}");
                    recv.dispatch(&cmds, users);
                }
            }
        }
    }
}

impl<'a> Consumer<'a, Result<Dispatcher>> for Commands<Resources> {
    /// consume
    ///
    /// consume the Commands<Resources> itself to generate a Dispatcher or throw out an Error,
    /// Dispatcher used to dispatch specified resources to all of the clients.
    fn consume(self, ext: Self::Ext) -> Result<Dispatcher> {
        match self {
            Commands::Single(r) => {
                let receiver = crate::gen_receiver!(
                    &r,
                    ext,
                    User,
                    Username,
                    ProfilePhoto,
                    Message,
                    ReadStatus,
                    Group,
                    Member,
                    Reviewer,
                    StreamSettings,
                    StreamLevel,
                    TopicSettings,
                    TopicLevel,
                    Task,
                    TaskId,
                    TaskReceipt
                )?;
                Ok(Dispatcher::Single(Box::new(Commands::Single(r)), receiver))
            }
            Commands::Multi(cmds) => {
                let mut updaters = Vec::new();
                for cmd in cmds {
                    let receiver = crate::gen_receiver!(
                        &cmd,
                        ext,
                        User,
                        Username,
                        ProfilePhoto,
                        Message,
                        ReadStatus,
                        Group,
                        Member,
                        Reviewer,
                        StreamSettings,
                        StreamLevel,
                        TopicSettings,
                        TopicLevel,
                        Task,
                        TaskId,
                        TaskReceipt
                    )?;
                    updaters.push((Box::new(Commands::Single(cmd)), receiver))
                }
                Ok(Dispatcher::Multi(updaters))
            }
        }
    }

    type Ext = crate::resources::Extension<'a>;
}

#[macro_export]
macro_rules! gen_receiver {
    ($r: expr, $ext: expr, $($items: ident),*) => {{
        match $r {
            $(
                Resources::$items(cmd) => cmd.generate($ext),
            )*
        }
    }};
}
