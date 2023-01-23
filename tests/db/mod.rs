#[cfg(test)]
mod tests {
    use anyhow::Result;
    use libgout::db::interface::Db;

    use crate::common::{generate_random_food, generate_random_foods, test_context::TestContext};

    #[tokio::test]
    async fn test_get_food() -> Result<()> {
        let test_context = TestContext::new().await;
        let db = test_context.context.database();

        assert!(db.get_food(5).await.is_err());

        let food = generate_random_food();
        let id = db.add_food(&food.clone().into()).await?;

        let db_food = db.get_food(id).await?;

        assert_eq!(db_food.name, food.name);
        assert_eq!(db_food.kcal, food.kcal);
        assert_eq!(db_food.purine, food.purine);
        assert_eq!(db_food.gout_factor, food.gout_factor);

        Ok(())
    }

    #[tokio::test]
    async fn test_insert() -> Result<()> {
        let test_context = TestContext::new().await;
        let db = test_context.context.database();

        let foods = generate_random_foods(10);

        for f in foods {
            db.add_food(&f.into()).await?;
        }

        let foods = db.get_foods().await?;
        assert_eq!(10, foods.len());

        let f1 = foods.first().unwrap();

        assert_eq!(f1.kcal, 1);
        assert_eq!(f1.purine, 4);
        assert_eq!(f1.uric_acid, Some(0.0));
        assert_eq!(f1.gout_factor, Some(2));

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<()> {
        let test_context = TestContext::new().await;
        let db = test_context.context.database();

        let food = generate_random_food();
        db.add_food(&food.into()).await?;

        let foods = db.get_foods().await?;
        assert_eq!(1, foods.len());

        db.delete_food(foods.first().unwrap().id).await?;
        let foods = db.get_foods().await?;
        assert_eq!(0, foods.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<()> {
        let test_context = TestContext::new().await;
        let db = test_context.context.database();

        let food = generate_random_food();
        db.add_food(&food.into()).await?;

        let mut foods = db.get_foods().await?;
        let f = foods.first_mut().unwrap();
        f.name = "updated_food".to_string();

        db.update_food(f.clone()).await?;
        let updated_food = db.get_food(f.id).await?;

        assert_eq!(updated_food.name, f.name);

        Ok(())
    }
}
