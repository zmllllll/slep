use std::ops::Deref;

use super::*;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "snake_case", type_name = "office_automation_task_type")]
pub(crate) enum TaskType {
    Private,
    Group,
    Unknown,
}

impl TaskType {
    pub(super) fn fmt(&self) -> String {
        match self {
            TaskType::Private => "private".to_owned(),
            TaskType::Group => "group".to_owned(),
            TaskType::Unknown => "unknown".to_owned(),
        }
    }
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for TaskType {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        use sqlx::{FromRow, Row};
        let res: TaskType = row.try_get("typ")?;
        sqlx::Result::Ok(res)
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub(crate) struct Task {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) des: Option<String>,
    pub(crate) typ: TaskType,
    pub(crate) consignor: i64,
    pub(crate) deadline: i64,
    pub(crate) timestamp: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub(crate) struct TaskReceipt {
    pub(crate) id: i64,
    pub(crate) receipts: Option<Receipts>,
}

#[derive(Debug, Serialize, sqlx::Decode, sqlx::Encode)]
pub(crate) struct Receipts(Vec<Receipt>);

impl Deref for Receipts {
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = Vec<Receipt>;
}

impl sqlx::Type<sqlx::Postgres> for Receipts {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Vec<Receipt> as sqlx::Type<sqlx::Postgres>>::type_info()
    }

    // fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    //     // sqlx::postgres::PgTypeInfo::with_name("name")
    //     <Receipt as sqlx::postgres::PgHasArrayType>::array_type_info()
    // }
}

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "_office_automation_task_receipt")]
pub(crate) struct Receipt {
    pub(crate) executor: i64,
    pub(crate) status: Status,
    pub(crate) des: String,
    pub(crate) timestamp: i64,
}

impl sqlx::postgres::PgHasArrayType for Receipt {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_office_automation_task_receipt")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "snake_case", type_name = "office_automation_task_status")]
pub(crate) enum Status {
    Unknown,
    Created,
    Pending,
    Confirmed,
    Blocked,
    Failed,
}

impl Status {
    pub(crate) fn fmt(&self) -> String {
        match self {
            Status::Unknown => "unknown".to_owned(),
            Status::Created => "created".to_owned(),
            Status::Pending => "pending".to_owned(),
            Status::Confirmed => "confirmed".to_owned(),
            Status::Blocked => "blocked".to_owned(),
            Status::Failed => "failed".to_owned(),
        }
    }
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for Status {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        use sqlx::{FromRow, Row};
        let res: Status = row.try_get("status")?;
        sqlx::Result::Ok(res)
    }
}
