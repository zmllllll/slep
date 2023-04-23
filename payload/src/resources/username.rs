use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "user",
    sqlite_table_name = "user",
    primary_key = "id:i64",
    constraint = "slep_user_pkey"
)]
pub struct Username {
    pub name: String,
    pub timestamp: i64,
}

impl Username {
    pub fn new(name: String) -> Self {
        Self {
            name,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for Username {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        let ids = gen_gid(1).await;
        Ok(ids[0])
    }
}
