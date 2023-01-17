mod common;

#[cfg(test)]
mod tests {
    use crate::common::{self, generate_random_food, generate_random_foods};
    use anyhow::Result;
    use libgout::db::interface::Db;

    #[tokio::test]
    async fn test_insert() -> Result<()> {
        let context = common::create_test_context().await;
        let db = context.database();

        let foods = generate_random_foods(10);
        for f in foods {
            db.add_food(&f.into()).await?;
        }

        let foods = db.get_foods().await?;
        assert_eq!(10, foods.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<()> {
        let context = common::create_test_context().await;
        let db = context.database();

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
        let context = common::create_test_context().await;
        let db = context.database();

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
