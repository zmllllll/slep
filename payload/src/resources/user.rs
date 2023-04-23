use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "user",
    sqlite_table_name = "user",
    primary_key = "id:i64",
    constraint = "slep_user_pkey"
)]
pub struct User {
    pub name: String,
    pub profile_picture: Option<String>,
    pub timestamp: i64,
}

impl User {
    pub fn new(name: String, profile_picture: Option<String>) -> Self {
        Self {
            name,
            profile_picture,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for User {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        let ids = gen_gid(1).await;
        Ok(ids[0])
    }
}
