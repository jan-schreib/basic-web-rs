#[cfg(test)]
mod tests {
    use anyhow::Result;
    use libgout::types::food::Food;

    use crate::common::{get, test_context::TestContext};

    #[tokio::test]
    async fn test_get_food() -> Result<()> {
        let tc = TestContext::new().await;
        tc.run();

        let res = get(tc.url() + "/api/foods").await?;

        let f = res.json::<Vec<Food>>().await.unwrap();

        assert_eq!(f.len(), 0);

        Ok(())
    }
}
