use super::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "lowercase")]
pub(crate) struct UserInfo {
    pub(crate) id: Option<i64>,
    pub(crate) name: String,
    pub(crate) profile_picture: Option<String>,
}
