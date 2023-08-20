use std::{str::FromStr, sync::Arc};

use sqlx::{
    postgres::PgPoolOptions,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};

use self::{interface::Db, postgres::Postgres, sqlite::Sqlite};

pub mod interface;
pub mod postgres;
pub mod sqlite;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DbError")]
    Db(#[from] sqlx::Error),
    #[error("MigrationError")]
    Migration(#[from] interface::Error),
    #[error("MutexError")]
    Mutex(#[from] tokio::sync::TryLockError),
    #[error("No entry with id `{0}`")]
    NotFound(i64),
}

#[derive(Clone, Debug)]
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

        //database.connection.clone().run_migrations().await?;

        Ok(database)
    }
}

impl Database<Postgres> {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

        let pg = Postgres::new(pool).await;
        let database = Database {
            connection: Arc::new(pg),
        };

        Ok(database)
    }
}
