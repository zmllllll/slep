use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "office_automation_task",
    sqlite_table_name = "office_automation_task",
    primary_key = "id:i64",
    constraint = "slep_office_automation_task_pkey"
)]
pub struct OATask {
    pub name: String,
    pub des: Option<String>,
    #[resource(name = "typ", typ = "slep.office_automation_task_type")]
    // unknown, group, private
    pub typ: String,
    pub consignor: i64,
    pub deadline: i64,
    pub timestamp: i64,
    // #[resource(name = "receipts", typ = "slep.office_automation_task_receipt")]
    // pub receipts: Option<TaskReceipt>,
}

impl OATask {
    pub fn new(
        name: String,
        des: Option<String>,
        typ: String,
        consignor: i64,
        deadline: i64,
        // receipts: Option<i16>,
    ) -> Self {
        Self {
            name,
            des,
            typ,
            consignor,
            deadline,
            // receipts: None,
            timestamp: gen_timestamp(),
            // receipts,
        }
    }
}

impl GenResourceID for OATask {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        let ids = gen_gid(1).await;
        Ok(ids[0])
    }
}
