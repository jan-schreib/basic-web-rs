use crate::Context;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("DbError")]
    DbError(#[from] sqlx::Error),
}

#[async_trait]
pub trait Db {
    async fn add_food(&self, context: &Context) -> Result<(), DbError>;
    async fn get_food(&self, context: &Context) -> Result<String, DbError>;
    async fn update_food(&self, context: &Context) -> Result<(), DbError>;
    async fn delete_food(&self, context: &Context) -> Result<(), DbError>;
}
