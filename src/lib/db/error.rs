use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("DbError")]
    DbError(#[from] sqlx::Error),
    #[error("Migration Error")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("MutexError")]
    Mutex(#[from] tokio::sync::TryLockError),
    #[error("No entry with id `{0}`")]
    NotFound(i64),
}
