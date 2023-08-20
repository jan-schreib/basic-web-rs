use crate::db::{sqlite::Sqlite, Database, Error as DbError};
use crate::types::config::Config;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error")]
    Db(#[from] DbError),
    #[error("Migration error")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}

type ThisDB = Sqlite;

#[derive(Clone, Debug)]
pub struct Context {
    pub config: Config,
    pub database: Database<ThisDB>,
    pub cache: Arc<Cache>,
}

#[derive(Clone, Debug)]
pub struct Cache {}

impl Context {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let database = Database::<ThisDB>::new(&config.db.db_url).await?;

        let cache = Cache {};
        let config = config.clone();

        Ok(Context {
            config,
            database,
            cache: Arc::new(cache),
        })
    }

    pub fn database(&self) -> Arc<ThisDB> {
        self.database.connection.clone()
    }
}
