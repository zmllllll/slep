use super::*;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct SqliteTaskReceipt {
    pub executor: i64,
    pub status: String,
    pub des: Option<String>,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct TaskReceipt {
    pub hashkey: i64,
    pub executor: i64,
    pub status: String,
    pub des: Option<String>,
    pub timestamp: i64,
}

impl Resource<sqlx::Postgres> for TaskReceipt {
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
        _id: &Option<Self::ResourceID>,
        _executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }

    async fn update<'e, 'c: 'e, E>(&'e self, id: &Self::ResourceID, executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "UPDATE slep.office_automation_task SET receipts = receipts 
            || ($1, $2, $3, $4)::slep.office_automation_task_receipt WHERE id = $5;";
        sqlx::query(sql)
            .bind(self.executor)
            .bind(&self.status)
            .bind(&self.des)
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

impl Resource<sqlx::Sqlite> for TaskReceipt {
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
        _id: &Option<Self::ResourceID>,
        _executor: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        todo!()
    }

    async fn update<'e, 'c: 'e, E>(&'e self, id: &Self::ResourceID, executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "INSERT INTO office_automation_task_receipt
        (task_id, executor, status, des, timestamp) 
        VALUES ($1, $2, $3, $4, $5);";

        sqlx::query(sql)
            .bind(id)
            .bind(self.executor)
            .bind(&self.status)
            .bind(&self.des)
            .bind(self.timestamp)
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

impl Resource<sqlx::Sqlite> for SqliteTaskReceipt {
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
        let sql = "INSERT INTO office_automation_task_receipt
        (task_id, executor, status, des, timestamp) 
        VALUES ($1, $2, $3, $4, $5);";

        sqlx::query(sql)
            .bind(id)
            .bind(self.executor)
            .bind(&self.status)
            .bind(&self.des)
            .bind(self.timestamp)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'e, 'c: 'e, E>(&'e self, id: &Self::ResourceID, executor: E) -> Result<()>
    where
        E: sqlx::Executor<'c, Database = Any>,
    {
        let sql = "INSERT INTO office_automation_task_receipt
        (task_id, executor, status, des, timestamp) 
        VALUES ($1, $2, $3, $4, $5);";

        sqlx::query(sql)
            .bind(id)
            .bind(self.executor)
            .bind(&self.status)
            .bind(&self.des)
            .bind(self.timestamp)
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

impl TaskReceipt {
    pub fn new(
        hashkey: i64,
        executor: i64,
        status: String,
        des: Option<String>,
        // receipts: Option<i16>,
    ) -> Self {
        Self {
            hashkey,
            executor,
            status,
            des,
            timestamp: gen_timestamp(),
            // receipts,
        }
    }
}

impl GenResourceID for TaskReceipt {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        let ids = gen_gid(1).await;
        Ok(ids[0])
    }
}

impl GenResourceID for SqliteTaskReceipt {
    type Target = i64;

    async fn gen_id() -> Result<i64> {
        let ids = gen_gid(1).await;
        Ok(ids[0])
    }
}
