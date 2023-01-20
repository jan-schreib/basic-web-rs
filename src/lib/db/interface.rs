use super::error::DbError;
use crate::types::food::{Food, FoodInsert};
use async_trait::async_trait;

#[async_trait]
pub trait Db {
    async fn add_food(&self, food: &FoodInsert) -> Result<i64, DbError>;
    async fn get_foods(&self) -> Result<Vec<Food>, DbError>;
    async fn get_food(&self, id: i64) -> Result<Food, DbError>;
    async fn update_food(&self, food: Food) -> Result<(), DbError>;
    async fn delete_food(&self, id: i64) -> Result<(), DbError>;
}
