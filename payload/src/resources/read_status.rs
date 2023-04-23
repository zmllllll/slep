use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum AddrType {
    Private { receiver: i64 },
    Stream { gid: i64, stream: String },
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Addr {
    pub uid: i64,
    #[serde(flatten)]
    pub addr_type: AddrType,
    pub topic: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ReadStatus {
    pub addr: Addr,
    pub latest_message_id: i64,
}

impl GenResourceID for ReadStatus {
    type Target = Addr;

    async fn gen_id() -> Result<Addr> {
        Err(anyhow::anyhow!("read status no id generation required"))
    }
}

impl Resource<sqlx::Postgres> for ReadStatus {
    type ResourceID = Addr;

    async fn insert<'e, 'c: 'e, E>(
        &'e self,
        _id: &Option<Self::ResourceID>,
        _executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }

    async fn upsert<'e, 'c: 'e, E>(
        &'e self,
        _id: &Option<Self::ResourceID>,
        executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql =
            "INSERT INTO slep.read_status ( addr, latest_message_id ) VALUES ( $1::JSONB, $2 ) 
        ON CONFLICT ON CONSTRAINT slep_read_status_pkey 
        DO UPDATE SET latest_message_id = EXCLUDED.latest_message_id";

        sqlx::query(sql)
            .bind(serde_json::to_string(&self.addr)?)
            .bind(self.latest_message_id)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'e, 'c: 'e, E>(&'e self, _id: &Self::ResourceID, _executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }

    async fn drop<'e, 'c: 'e, E>(_id: &Self::ResourceID, _executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }
}

impl Resource<sqlx::Sqlite> for ReadStatus {
    type ResourceID = Addr;

    async fn insert<'e, 'c: 'e, E>(
        &'e self,
        _id: &Option<Self::ResourceID>,
        _executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }

    async fn upsert<'e, 'c: 'e, E>(
        &'e self,
        _id: &Option<Self::ResourceID>,
        executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "INSERT INTO read_status ( addr, latest_message_id ) VALUES ( $1, $2 ) 
        ON CONFLICT (addr) 
        DO UPDATE SET latest_message_id = EXCLUDED.latest_message_id";

        sqlx::query(sql)
            .bind(serde_json::to_string(&self.addr)?)
            .bind(self.latest_message_id)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'e, 'c: 'e, E>(&'e self, _id: &Self::ResourceID, _executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }

    async fn drop<'e, 'c: 'e, E>(_id: &Self::ResourceID, _executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }
}
