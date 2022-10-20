mod context;

#[cfg(test)]
mod tests {
    use crate::context::tests::*;

    #[tokio::test]
    async fn test_initialization() -> anyhow::Result<()> {
        let context = IntegrationTestContext::new().await?;
        let greet = context.get_greeting().await?;
        assert_eq!(greet, INITIAL_GREETING);
        Ok(())
    }
}
