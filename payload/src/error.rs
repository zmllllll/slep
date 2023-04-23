#[derive(serde::Serialize, thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("{0:?} user not exist")]
    UserNotExist(String),
    #[error("system error: {0:?}")]
    System(String),
    #[error("params cant be none: {0:?}")]
    ParamsCantBeNone(String),
    #[error("params cant be none at the same time: {0:?}")]
    BadRequest(String),
    #[error("{0:?} {1:?} execute failed")]
    CommandExecuteFailed(String, String),
    #[error("channel send error: {0:?}")]
    ChannelSendFailed(String),
    #[error("database create failed: {0:?}")]
    DatabaseCreateFailed(String),
    #[error("[{0:?}] pool create failed: {1:?}")]
    PoolCreateFailed(String, String),
    #[error("[{0:?}] pool get failed")]
    PoolGetFailed(String),
    #[error("[{0:?}] query failed: {1:?}")]
    DataQueryFailed(String, String),
    #[error("[{0:?}] insert failed: {1:?}")]
    DataInsertFailed(String, String),
    #[error("[{0:?}] update failed: {1:?}")]
    DataUpdateFailed(String, String),
    #[error("migrate error: {0:?}")]
    MigrateError(String),
    #[error("notify send error: {0:?}")]
    NotifySendError(String),
    #[error("insufficient permissions: current: {0:?}, needed: {1:?}")]
    InsufficientPermissions(i16, i16),
    // #[error(transparent)]
    // PublicError(#[from] anyhow::Error),
}
