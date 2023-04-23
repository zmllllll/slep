use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "topic_settings",
    sqlite_table_name = "topic_settings",
    primary_key = "hashkey:i64",
    constraint = "slep_topic_settings_pkey"
)]
pub struct TopicLevel {
    pub rlevel: i16,
    pub wlevel: i16,
    pub timestamp: i64,
}

impl TopicLevel {
    pub fn new(rlevel: i16, wlevel: i16) -> Self {
        Self {
            rlevel,
            wlevel,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for TopicLevel {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        let ids = gen_gid(1).await;
        Ok(ids[0])
    }
}
