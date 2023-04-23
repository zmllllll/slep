use super::*;
use sqlx::sqlite::SqliteQueryResult;
pub(super) trait Query: Serialize + Sized + std::fmt::Debug {
    async fn query<F, O>(op: F) -> Result<Vec<Self>, error::Error>
    where
        F: FnOnce(&'static sqlx::Pool<sqlx::Any>) -> O,
        O: std::future::Future<Output = Result<Vec<Self>, sqlx::Error>>,
    {
        if let Some(pool) = unsafe { SQLITE_POOL.get() } {
            let res = op(pool)
                .await
                .map_err(|e| error::Error::DataQueryFailed("sqlite".to_owned(), e.to_string()))?;
            // tracing::info!("query result: {res:?}");
            Ok(res)
        } else {
            Err(error::Error::PoolGetFailed("sqlite".to_owned()))
        }
    }
}
impl<T: Serialize + Sized + std::fmt::Debug> sqlite_operator::Query for T {}

pub(super) trait SerdeTool: Serialize + Sized {
    fn serde_to_string(self) -> Result<String, error::Error> {
        serde_json::to_string(&self).map_err(|e| error::Error::System(e.to_string()))
    }

    fn map<F, B>(self, op: F) -> B
    where
        F: FnOnce(Self) -> B,
    {
        op(self)
    }
}

impl<T: Serialize + Sized> sqlite_operator::SerdeTool for T {}
