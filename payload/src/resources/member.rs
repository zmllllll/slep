use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "group_member",
    sqlite_table_name = "group_member",
    primary_key = "uid:i64, gid:i64",
    constraint = "slep_group_member_pkey"
)]
pub struct GroupMember {
    pub level: i16,
    pub timestamp: i64,
}

impl GroupMember {
    pub fn new(level: i16) -> Self {
        Self {
            level,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for GroupMember {
    type Target = (i64, i64);

    async fn gen_id() -> Result<(i64, i64)> {
        let ids = gen_gid(2).await;
        Ok((ids[0], ids[1]))
    }
}
