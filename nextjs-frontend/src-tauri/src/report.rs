use std::collections::HashSet;

use crate::{
    notify::{Notifies, Notify, Payload},
    resources::Resources,
};

use super::*;

pub(super) trait Reporter {
    fn handle(self) -> Notifies;
}

impl Reporter for Commands<Resources> {
    fn handle(self) -> Notifies {
        use notify::Deliver as _;
        match self {
            Commands::Single(r) => Notifies::Single(pack!(
                r,
                User,
                Username,
                ProfilePicture,
                Message,
                Group,
                Member,
                Reviewer,
                StreamSettings,
                StreamLevel,
                TopicSettings,
                TopicLevel,
                Task,
                TaskId,
                TaskReceipt,
                ReadStatus
            )),
            Commands::Multi(rs) => {
                let mut notifies = Vec::new();
                for r in rs {
                    notifies.push(pack!(
                        r,
                        User,
                        Username,
                        ProfilePicture,
                        Message,
                        Group,
                        Member,
                        Reviewer,
                        StreamSettings,
                        StreamLevel,
                        TopicSettings,
                        TopicLevel,
                        Task,
                        TaskId,
                        TaskReceipt,
                        ReadStatus
                    ))
                }
                Notifies::Multi(notifies)
            }
        }
    }
}

#[macro_export]
macro_rules! pack {
    ($r: expr, $($items: ident),*) => {{
        match $r {
            $(
                Resources::$items(cmd) => Notify::pack_data(cmd),
            )*
        }
    }};
}
