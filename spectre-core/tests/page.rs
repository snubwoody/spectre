use spectre_core::Result;
use spectre_core::browser::Browser;

#[tokio::test]
async fn get_by_name() -> Result<()> {
    let mut browser = Browser::launch().await?;
    let mut page = browser.goto("https://google.com").await?;
    page.get_by_name().await?;

    Ok(())
}
