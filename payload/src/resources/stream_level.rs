use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, resource_macros::Resource)]
#[resource(
    schema_name = "slep",
    pg_table_name = "stream_settings",
    sqlite_table_name = "stream_settings",
    primary_key = "stream:String, gid:i64",
    constraint = "slep_stream_settings_pkey"
)]
pub struct StreamLevel {
    pub rlevel: i16,
    pub wlevel: i16,
    pub timestamp: i64,
}

impl StreamLevel {
    pub fn new(rlevel: i16, wlevel: i16) -> Self {
        Self {
            rlevel,
            wlevel,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for StreamLevel {
    type Target = (String, i64);

    async fn gen_id() -> Result<(String, i64)> {
        let ids = gen_gid(2).await;
        Ok((ids[0].to_string(), ids[1]))
    }
}
