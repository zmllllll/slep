use super::*;
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(super) struct Message {
    pub(crate) id: i64,
    pub(crate) gid: Option<i64>,
    pub(crate) typ: MessageType,
    pub(crate) addr: String,
    pub(crate) topic: String,
    pub(crate) content: String,
    pub(crate) sender: i64,
    pub(crate) timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "snake_case", type_name = "message_type")]
pub(super) enum MessageType {
    /// 创建
    Md,
    /// 添加
    Img,
    /// 更新
    Video,
    /// 删除
    Audio,
    /// 删除
    Bot,
    /// 任命
    Unknown,
}

impl MessageType {
    pub(super) fn fmt(&self) -> String {
        match self {
            MessageType::Md => "md".to_owned(),
            MessageType::Img => "img".to_owned(),
            MessageType::Video => "video".to_owned(),
            MessageType::Audio => "audio".to_owned(),
            MessageType::Bot => "bot".to_owned(),
            MessageType::Unknown => "unknown".to_owned(),
        }
    }
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for MessageType {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        use sqlx::{FromRow, Row};
        let res: MessageType = row.try_get("typ")?;
        sqlx::Result::Ok(res)
    }
}
