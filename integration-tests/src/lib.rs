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

    #[tokio::test]
    #[should_panic]
    async fn test_fail_cross_get_greeting_as_view() {
        let context = IntegrationTestContext::new().await.unwrap();
        context.cross_get_greeting_as_view().await.unwrap();
    }

    #[tokio::test]
    async fn test_cross_set_greeting() -> anyhow::Result<()> {
        let context = IntegrationTestContext::new().await?;
        context.cross_set_greeting().await?;
        let greet = context.cross_get_greeting().await?;
        assert_eq!(greet, GREETING1);
        Ok(())
    }

    #[tokio::test]
    async fn test_rewards_can_be_called_as_view() -> anyhow::Result<()> {
        let context = IntegrationTestContext::new().await?;
        let greet = context.can_be_called_as_view().await?;
        assert_eq!(greet, "Sure");
        Ok(())
    }

    #[tokio::test]
    async fn test_cross_get_greeting_low_level() -> anyhow::Result<()> {
        let context = IntegrationTestContext::new().await?;
        let greet = context.cross_get_greeting_low_level().await?;
        println!("Cross contract greet = \"{greet}\"");
        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn test_cross_get_greeting_low_level_as_view() {
        let context = IntegrationTestContext::new().await.unwrap();
        let greet = context
            .cross_get_greeting_low_level_as_view()
            .await
            .unwrap();
        println!("Cross contract greet = \"{greet}\"");
    }
}
