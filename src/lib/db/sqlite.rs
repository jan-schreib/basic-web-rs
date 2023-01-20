use std::sync::Arc;

use crate::types::food::{Food, FoodInsert};

use super::{error::DbError, interface::Db};
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::Row;
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
    async fn add_food(&self, food: &FoodInsert) -> Result<i64, DbError> {
        let mut conn = self.get_db().acquire().await?;
        let res = sqlx::query(
            "insert into foods(name, kcal, purine, uric_acid, gout_factor) values (?, ?, ?, ?, ?) returning id",
        )
        .bind(food.name.clone())
        .bind(food.kcal)
        .bind(food.purine)
        .bind(food.uric_acid.unwrap_or_default())
        .bind(food.gout_factor.unwrap_or_default())
        .fetch_one(&mut conn)
        .await?;

        let id = res.get("id");

        Ok(id)
    }

    async fn get_foods(&self) -> Result<Vec<Food>, DbError> {
        let mut conn = self.get_db().acquire().await?;

        let foods = sqlx::query_as::<_, Food>("select * from foods")
            .fetch_all(&mut conn)
            .await?;

        Ok(foods)
    }

    async fn get_food(&self, id: i64) -> Result<Food, DbError> {
        let mut conn = self.get_db().acquire().await?;

        let food = sqlx::query_as::<_, Food>("select * from foods where id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await?;

        if let Some(food) = food {
            Ok(food)
        } else {
            Err(DbError::NotFound(id))
        }
    }
    async fn update_food(&self, food: Food) -> Result<(), DbError> {
        let mut conn = self.get_db().acquire().await?;

        sqlx::query("update foods set name = ? where id = ?")
            .bind(food.name)
            .bind(food.id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
    async fn delete_food(&self, id: i64) -> Result<(), DbError> {
        let mut conn = self.get_db().acquire().await?;

        sqlx::query("delete from foods where id = ?")
            .bind(id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
