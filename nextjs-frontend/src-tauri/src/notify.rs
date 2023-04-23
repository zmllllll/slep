use super::{error::Error, Command, UnboundedSender};

#[derive(Debug)]
pub(super) enum Notify {
    /// event, data
    SendResponse(String, Option<Payload>),
}

pub(super) enum Notifies {
    Single(Notify),
    Multi(Vec<Notify>),
}

impl Notifies {
    pub(super) fn send(self) {
        use Deliver as _;
        match self {
            Notifies::Single(n) => n.send(),
            Notifies::Multi(ns) => {
                for n in ns {
                    n.send()
                }
            }
        }
    }

    pub(super) fn data(tag: &str, data: Option<Payload>) -> Self {
        Notifies::Single(Notify::SendResponse(tag.to_owned(), data))
    }
}

pub(super) trait Deliver {
    fn pack_data<R>(cmd: Command<resource::GeneralAction<sqlx::Sqlite, R>>) -> Self
    where
        R: resource::Resource<sqlx::Sqlite>;
    fn send(self);
}

impl Deliver for Notify {
    fn pack_data<R>(cmd: Command<resource::GeneralAction<sqlx::Sqlite, R>>) -> Self
    where
        R: resource::Resource<sqlx::Sqlite>,
    {
        use Info as _;
        let data = serde_json::to_string(&cmd.action).unwrap();
        Self::SendResponse(cmd.tag().to_string(), Some(Payload::Data { data }))
    }

    fn send(self) {
        if let Err(e) = crate::NOTIFY_TX.get().unwrap().send(self) {
            tracing::error!("notify send err: {e}");
        };
    }
}

trait Info<A> {
    fn fmt(&self) -> i64;

    fn tag(&self) -> &String;
}

impl<R> Info<R> for Command<resource::GeneralAction<sqlx::Sqlite, R>>
where
    R: resource::Resource<sqlx::Sqlite>,
{
    fn fmt(&self) -> i64 {
        self.trace
    }

    fn tag(&self) -> &String {
        &self.tag
    }
}

#[derive(Debug, serde::Serialize, Clone)]
#[serde(untagged)]
pub(crate) enum Payload {
    Data { data: String },
    Error { trace_id: String, error: Error },
}

impl Payload {
    pub(crate) fn data<T: serde::Serialize>(data: T) -> Self {
        Payload::Data {
            data: serde_json::to_string(&data).unwrap(),
        }
    }

    pub(crate) fn err(trace_id: i64, error: Error) -> Self {
        Payload::Error {
            trace_id: trace_id.to_string(),
            error,
        }
    }
}

pub(super) async fn handle(
    mut notify_rx: tokio_stream::wrappers::UnboundedReceiverStream<Notify>,
    window: tauri::Window,
) {
    use tokio_stream::StreamExt as _;
    while let Some(notify) = notify_rx.next().await {
        match notify {
            Notify::SendResponse(ref event, res) => {
                if let Err(e) = window.emit(event, res.clone()) {
                    tracing::error!("event send error: {e}");
                } else {
                    tracing::info!("send event successfully: {event}");
                }
            }
        }
    }
}
