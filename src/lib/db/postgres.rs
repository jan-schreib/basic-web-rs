use super::interface::{Db, Error};
use async_trait::async_trait;
use std::sync::Arc;

use sqlx::{PgPool, Pool};

#[derive(Clone, Debug)]
pub struct Postgres(Arc<PgPool>);

impl Postgres {
    pub async fn new(pool: Pool<sqlx::Postgres>) -> Self {
        Postgres(Arc::new(pool))
    }

    pub fn get_db(&self) -> Arc<Pool<sqlx::Postgres>> {
        self.0.clone()
    }
}

#[async_trait]
impl Db for Postgres {
    async fn run_migrations(&self) -> Result<(), Error> {
        sqlx::migrate!().run(&*self.0).await?;

        Ok(())
    }
}
