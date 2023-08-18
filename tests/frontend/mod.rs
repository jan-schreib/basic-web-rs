#[cfg(test)]
mod tests {
    use crate::common::{get, test_context::TestContext};
    use anyhow::Result;

    #[tokio::test]
    async fn test_status() -> Result<()> {
        let tc = TestContext::new().await;
        tc.run();

        let res = get(tc.url() + "/").await?;
        assert_eq!(res.status(), 200);

        let body = res.text().await?;

        assert!(body.contains("<html"));
        assert!(body.contains("</html>"));

        Ok(())
    }
}
