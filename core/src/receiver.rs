use std::collections::{HashMap, HashSet};

use resource::Commands;

use crate::{event::ConnectionEvent, resources::Resources};

pub type Recv = i64;
#[derive(Debug)]
pub enum Receiver {
    None,
    Single(Recv),
    List(HashSet<Recv>),
}

impl Receiver {
    pub(crate) fn dispatch(
        self,
        cmds: &Commands<Resources>,
        users: &mut HashMap<i64, tokio::sync::mpsc::UnboundedSender<ConnectionEvent>>,
    ) {
        self.map(cmds, |recv, cmds| {
            if let Some(sender) = users.get_mut(&recv) {
                match sender.send(ConnectionEvent::Message(
                    serde_json::to_string(&cmds).unwrap(),
                )) {
                    Ok(_) => {
                        tracing::info!("ready to send report: {cmds:?}")
                    }
                    Err(e) => tracing::info!("send task error: {e}"),
                };
            }
        });
    }

    pub(self) fn map<F>(self, cmds: &Commands<Resources>, mut f: F)
    where
        F: FnMut(Recv, &Commands<Resources>),
    {
        match self {
            Receiver::None => (),
            Receiver::Single(recv) => f(recv, cmds),
            Receiver::List(recvs) => {
                for recv in recvs.into_iter() {
                    f(recv, cmds)
                }
            }
        }
    }
}

impl From<HashSet<i64>> for Receiver {
    fn from(value: HashSet<i64>) -> Self {
        Receiver::List(value)
    }
}

impl From<i64> for Receiver {
    fn from(value: i64) -> Self {
        Receiver::Single(value)
    }
}
