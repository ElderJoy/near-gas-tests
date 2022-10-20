mod context;

#[cfg(test)]
mod tests {
    use crate::context::tests::*;

    #[tokio::test]
    async fn test_initialization() -> anyhow::Result<()> {
        IntegrationTestContext::new().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_set_greeting() -> anyhow::Result<()> {
        let context = IntegrationTestContext::new().await?;
        assert_eq!(context.get_greeting().await?, INITIAL_GREETING);
        context.set_greeting().await?;
        assert_eq!(context.get_greeting().await?, GREETING1);
        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn test_fail_set_greeting_as_view() {
        let context = IntegrationTestContext::new().await.unwrap();
        context.set_greeting_as_view().await.unwrap();
    }

    #[tokio::test]
    async fn test_cross_get_greeting() -> anyhow::Result<()> {
        let context = IntegrationTestContext::new().await?;
        let greet = context.cross_get_greeting().await?;
        println!("Cross contract greet = \"{greet}\"");
        Ok(())
    }
}
