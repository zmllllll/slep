use super::*;

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "message",
    sqlite_table_name = "message",
    primary_key = "id:i64",
    constraint = "slep_message_pkey"
)]
pub struct Message {
    pub gid: Option<i64>,
    #[resource(name = "typ", typ = "slep.message_type")]
    pub typ: String,
    pub addr: String,
    pub topic: String,
    pub content: String,
    pub sender: i64,
    pub timestamp: i64,
}

impl Message {
    pub fn new(
        gid: Option<i64>,
        typ: String,
        addr: String,
        topic: String,
        content: String,
        sender: i64,
    ) -> Self {
        Self {
            gid,
            typ,
            addr,
            topic,
            content,
            sender,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for Message {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        gen_gid(1).await.pop().ok_or(anyhow::anyhow!("0"))
    }
}
