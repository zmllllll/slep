use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct TaskId {
    pub associate_task_id: i64,
    pub gid: Option<i64>,
    pub addr: String,
    pub topic: String,
    pub timestamp: i64,
}

impl TaskId {
    pub fn new(associate_task_id: i64, gid: Option<i64>, addr: String, topic: String) -> Self {
        Self {
            associate_task_id,
            gid,
            addr,
            topic,
            timestamp: gen_timestamp(),
        }
    }
}

impl GenResourceID for TaskId {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        let ids = gen_gid(1).await;
        Ok(ids[0])
    }
}

impl Resource<sqlx::Postgres> for TaskId {
    type ResourceID = i64;

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
        id: &Option<Self::ResourceID>,
        executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "INSERT INTO
        slep.topic_settings (hashkey, associate_task_id, timestamp)
      VALUES
        ($1, $2, $3) ON CONFLICT ON CONSTRAINT slep_topic_settings_pkey DO
      UPDATE
      SET
        hashkey = EXCLUDED.hashkey,
        associate_task_id = EXCLUDED.associate_task_id,
        timestamp = EXCLUDED.timestamp";
        sqlx::query(sql)
            .bind(id)
            .bind(self.associate_task_id)
            .bind(self.timestamp)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'e, 'c: 'e, E>(&'e self, id: &Self::ResourceID, executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "UPDATE slep.topic_settings SET 
        associate_task_id = $1, 
        timestamp = $2
        WHERE hashkey = $3;";
        sqlx::query(sql)
            .bind(self.associate_task_id)
            .bind(self.timestamp)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn drop<'e, 'c: 'e, E>(_id: &Self::ResourceID, _executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }
}

impl Resource<sqlx::Sqlite> for TaskId {
    type ResourceID = i64;

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
        id: &Option<Self::ResourceID>,
        executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "INSERT INTO
        topic_settings (hashkey, associate_task_id, timestamp)
      VALUES
        ($1, $2, $3) ON CONFLICT (hashkey) DO UPDATE
      SET
        hashkey = EXCLUDED.hashkey,
        associate_task_id = EXCLUDED.associate_task_id,
        timestamp = EXCLUDED.timestamp";
        sqlx::query(sql)
            .bind(id)
            .bind(self.associate_task_id)
            .bind(self.timestamp)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'e, 'c: 'e, E>(&'e self, id: &Self::ResourceID, executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "UPDATE slep.topic_settings SET 
        associate_task_id = $1, 
        timestamp = $2
        WHERE hashkey = $3;";
        sqlx::query(sql)
            .bind(self.associate_task_id)
            .bind(self.timestamp)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn drop<'e, 'c: 'e, E>(_id: &Self::ResourceID, _executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }
}
