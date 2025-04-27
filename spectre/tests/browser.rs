use spectre::{Browser, Result};

#[tokio::test]
async fn create_page() -> Result<()> {
    let mut browser = Browser::start().await?;
    let _ = browser.goto("https://youtube.com").await?;
    Ok(())
}
