use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Migration Error")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

#[async_trait]
pub trait Db {
    async fn run_migrations(&self) -> Result<(), Error>;
}
