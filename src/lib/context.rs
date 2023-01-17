use crate::db::{interface::Db, sqlite::Sqlite};
use crate::gtypes::config::GConfig;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};
use std::{str::FromStr, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in Sqlx")]
    SqlxError(#[from] sqlx::Error),
    #[error("Migration error")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}

pub struct Context {
    pub config: GConfig,
    pub database: Database<Sqlite>,
    pub cache: Arc<Cache>,
}

pub struct Database<T: Db> {
    pub connection: Arc<T>,
}

impl Database<Sqlite> {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let conn_opts = SqliteConnectOptions::from_str(url)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .read_only(false);

        let conn = SqlitePool::connect_with(conn_opts).await?;

        let sqlite = Sqlite::new(conn);
        let database = Database {
            connection: Arc::new(sqlite),
        };

        Ok(database)
    }
}

pub struct Cache {}

impl Context {
    pub async fn new(config: &GConfig) -> Result<Self, Error> {
        let database = Database::new(&config.db_url).await?;
        let cache = Cache {};
        let config = config.clone();
        Ok(Context {
            config,
            database,
            cache: Arc::new(cache),
        })
    }

    pub async fn run_migrations(&self) -> Result<(), Error> {
        sqlx::migrate!()
            .run(&*self.database.connection.get_db())
            .await?;

        Ok(())
    }

    pub fn database(&self) -> Arc<Sqlite> {
        self.database.connection.clone()
    }
}
