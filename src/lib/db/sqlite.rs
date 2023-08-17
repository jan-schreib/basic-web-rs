use std::sync::Arc;

use super::interface::{Db, Error};
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct Sqlite(Arc<SqlitePool>);

impl Sqlite {
    pub fn new(pool: Pool<sqlx::Sqlite>) -> Self {
        Self(Arc::new(pool))
    }

    pub fn get_db(&self) -> Arc<Pool<sqlx::Sqlite>> {
        self.0.clone()
    }
}

#[async_trait]
impl Db for Sqlite {
    async fn run_migrations(&self) -> Result<(), Error> {
        sqlx::migrate!().run(&*self.0).await?;

        Ok(())
    }
}
