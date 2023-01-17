use std::sync::Arc;

use crate::gtypes::food::{Food, FoodInsert};

use super::{error::DbError, interface::Db};
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::SqlitePool;

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
    async fn add_food(&self, food: &FoodInsert) -> Result<(), DbError> {
        let mut conn = self.get_db().acquire().await?;

        let res = sqlx::query("insert into foods(name) values (?)")
            .bind(food.name.clone())
            .execute(&mut conn)
            .await?;

        assert!(res.rows_affected() == 1);

        Ok(())
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
            .fetch_one(&mut conn)
            .await?;

        Ok(food)
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
