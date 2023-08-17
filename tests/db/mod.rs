#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::common::test_context::TestContext;

    #[tokio::test]
    async fn test_get_realm() -> Result<()> {
        let test_context = TestContext::new().await;
        let _db = test_context.context.database();
        Ok(())
    }
}
