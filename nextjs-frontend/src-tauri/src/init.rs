use super::*;
use anyhow::Ok;

mod message;
pub(crate) mod pull;
mod task;

// init database & update local data
pub(crate) async fn init(uid: i64, db: &config::DB) -> Result<()> {
    let app_dir = STORAGE
        .get()
        .and_then(|dir| dir.to_str())
        .ok_or(error::Error::System("app dir not exist".to_string()))?;

    let sqlite_addr = format!("sqlite://{app_dir}/{uid}.db");
    tracing::error!("sqlite_addr: {}", sqlite_addr);
    let sqlite_addr = sqlite_addr.as_str();
    init_database(sqlite_addr, &db.pg_url)
        .await
        .map_err(|e| error::Error::System(format!("init database error: {}", e)))?;
    tokio::task::spawn(async move {
        use core::result::Result::Ok;
        match pulling(uid).await {
            Ok(_) => {
                tracing::info!("pulling data successful!");
                use notify::Deliver as _;
                notify::Notify::SendResponse("Initialize".to_string(), None).send();
            }
            Err(e) => tracing::info!("pulling data error: {e}"),
        };
        // command::user::complement_user_info(uid).await;
    });
    Ok(())
}

pub(crate) mod config {
    use super::*;
    #[derive(Debug, Deserialize)]
    struct Config {
        db: DB,
        url: String,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct DB {
        pub(crate) pg_url: String,
    }

    pub(crate) fn read_configure() -> (DB, String) {
        let yaml = include_str!("../config.yaml");
        let config = serde_yaml::from_str(yaml).expect("app.yaml read failed!");
        let Config { db, url } = config;
        (db, url)
    }
}

pub(super) async fn pulling(uid: i64) -> Result<()> {
    tracing::debug!(">>>>>>>>>>>> pulling {uid} <<<<<<<<<<<<<<<");
    let (groups, members, levels) = pull::group_member(uid).await?;
    pull::user(&members).await?;
    pull::group(&groups).await?;
    pull::stream_settings(&groups).await?;
    pull::task(uid).await?;
    pull::task_receipt(uid).await?;
    let topics = pull::message(uid, levels).await?;
    pull::topic_settings(&topics).await?;
    Ok(())
}

async fn connect_pg(pg_url: &str) -> Result<sqlx::Pool<sqlx::Postgres>> {
    sqlx::Pool::<sqlx::Postgres>::connect(pg_url)
        .await
        .map_err(|e| {
            anyhow!(error::Error::PoolCreateFailed(
                "postgres".to_owned(),
                e.to_string()
            ))
        })
}

pub(crate) async fn init_database(sqlite_url: &str, pg_url: &str) -> Result<()> {
    async fn _create_database(sqlite_url: &str) -> Result<()> {
        Ok(sqlx::Sqlite::create_database(sqlite_url)
            .await
            .map_err(|e| error::Error::DatabaseCreateFailed(e.to_string()))?)
    }

    use sqlx::migrate::MigrateDatabase as _;
    if !sqlx::Sqlite::database_exists(sqlite_url)
        .await
        .unwrap_or(false)
    {
        _create_database(sqlite_url).await?
    };
    let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect(sqlite_url).await?;
    // sqlx::sqlite::SqlitePoolOptions::new()
    let sqlite_pool = if sqlx::migrate!("./migrations")
        .run(&sqlite_pool)
        .await
        .is_err()
    {
        tracing::error!("migrate filed: remove files & create database again");
        sqlite_pool.close().await;
        let storage = STORAGE.get().unwrap();
        sqlx::Sqlite::drop_database(sqlite_url).await?;
        _create_database(sqlite_url).await?;

        let sqlite_pool = sqlx::Pool::<sqlx::Any>::connect(sqlite_url).await?;
        sqlx::migrate!("./migrations").run(&sqlite_pool).await?;
        sqlite_pool
    } else {
        sqlite_pool
    };
    unsafe {
        SQLITE_POOL.take();
        let _ = SQLITE_POOL.set(sqlite_pool);
    }

    let pg_pool = connect_pg(pg_url).await?;

    let _ = PG_POOL.set(pg_pool);
    Ok(())
}

pub(crate) fn save_data() {}

pub(crate) fn init_log() -> Result<()> {
    // if cfg!()
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        // .with(fmt::layer())
        .with(formatting_layer)
        .init();
    color_eyre::install().map_err(|e| anyhow!(error::Error::System(e.to_string())))?;
    Ok(())
}
