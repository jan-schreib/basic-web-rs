#[cfg(test)]
mod tests {
    use anyhow::Result;
    use libgout::types::food::{Food, FoodInsert};

    use crate::common::{get, test_context::TestContext, delete, generate_random_food, post};

    #[tokio::test]
    async fn test_crate_food() -> Result<()> {
        let tc = TestContext::new().await;
        tc.run();

        let food: FoodInsert = generate_random_food().into();

        let res = post(tc.url() + "/api/food", &food).await?;

        assert_eq!(res.status(), 201);

        Ok(())
    }


    #[tokio::test]
    async fn test_get_food() -> Result<()> {
        let tc = TestContext::new().await;
        tc.run();

        let res = get(tc.url() + "/api/foods").await?;
        let foods = res.json::<Vec<Food>>().await.unwrap();

        assert_eq!(foods.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_food() -> Result<()> {
        let tc = TestContext::new().await;
        tc.run();

        let res = delete(tc.url() + "/api/foods/1").await?;
        assert_eq!(res.status(), 204);

        Ok(())
    }
}
