use super::{
    gen_topic_key, Command, Commands, Deserialize, GeneralAction, Serialize, TungMessage,
    CHANNEL_SENDER,
};
use crate::{
    resources::Resources,
    sqlite_operator::{Query, SerdeTool},
};
use payload::{
    resources,
    {error::Error, resources::gen_id},
};

pub(super) mod group;
pub(super) mod member;
pub(super) mod message;
pub(super) mod office_automation_task;
mod read_status;
pub(super) mod reviewer;
pub(super) mod stream;
pub(super) mod system;
pub(super) mod task_receipt;
pub(super) mod topic;
pub(super) mod user;

async fn send(msg: TungMessage) -> Result<(), Error> {
    let sender = CHANNEL_SENDER.read().await;
    if let Some(sender) = &sender.0 {
        tracing::info!("通道是否关闭：{}", sender.is_closed());
        match sender.send(msg) {
            Ok(_) => {
                // console_log!("websocket send: {}", text)
                // info!("channel send {text} successfully");
                drop(sender);
                Ok(())
            }
            Err(err) => {
                // console_log!("error sending ack: {:?}", err)
                tracing::error!("channel send cmd error: {err:?}");
                drop(sender);
                Err(Error::ChannelSendFailed(err.to_string()))
            }
        }
    } else {
        drop(sender);

        Err(Error::System("CHANNEL_SENDER is none".to_string()))
    }
}

trait Transform<D> {
    fn transform(self) -> Result<D, Error>;
}

impl Transform<Option<i64>> for Option<&str> {
    fn transform(self) -> Result<Option<i64>, Error> {
        if let Some(raw) = self {
            match raw.parse::<i64>() {
                Ok(d) => Ok(Some(d)),
                Err(e) => Err(Error::System(e.to_string())),
            }
        } else {
            Ok(None)
        }
    }
}

impl Transform<i64> for &str {
    fn transform(self) -> Result<i64, Error> {
        self.parse::<i64>()
            .map_err(|e: std::num::ParseIntError| Error::System(e.to_string()))
    }
}
