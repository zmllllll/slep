use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "user_group",
    sqlite_table_name = "user_group",
    primary_key = "id:i64",
    constraint = "slep_user_group_pkey"
)]
pub struct Group {
    pub pid: Option<i64>,
    pub name: String,
    pub des: Option<String>,
    pub timestamp: i64,
}

impl Group {
    pub fn new(pid: Option<i64>, name: String, des: Option<String>) -> Self {
        Self {
            pid,
            name,
            des,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for Group {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        gen_gid(1).await.pop().ok_or(anyhow::anyhow!("0"))
    }
}
