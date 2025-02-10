use std::error::Error;
use easee_rs::Client;

#[tokio::test]
async fn test_login() -> Result<(), Box<dyn Error + Send + Sync>> {
    Client::setup_with_username_and_password("username", "password").await?;
    Ok(())
}