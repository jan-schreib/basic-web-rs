use crate::gtypes::config::GConfig;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, Connection, SqliteConnection,
};
use std::{str::FromStr, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in Sqlx")]
    SqlxError(#[from] sqlx::Error),
}

pub struct Context {
    pub database: Arc<Database<SqliteConnection>>,
    pub cache: Arc<Cache>,
}

pub struct Database<T: Connection> {
    pub connection: T,
}

impl Database<SqliteConnection> {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let conn = SqliteConnectOptions::from_str(url)?
            .journal_mode(SqliteJournalMode::Wal)
            .read_only(false)
            .connect()
            .await?;

        let database = Database { connection: conn };

        Ok(database)
    }
}

pub struct Cache {}

impl Context {
    pub async fn new(config: &GConfig) -> Result<Self, Error> {
        let database = Database::new(&config.db_url).await?;
        let cache = Cache {};
        Ok(Context {
            database: Arc::new(database),
            cache: Arc::new(cache),
        })
    }
}
