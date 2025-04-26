use spectre::Browser;
use spectre::dom::NodeName;
use spectre::{EMPTY_PAGE, Result};

#[tokio::test]
async fn get_by_name() -> Result<()> {
    let mut browser = Browser::launch().await?;
    let mut page = browser.goto("https://google.com").await?;

    let root = page.get_by_name(NodeName::Document).await?;
    assert!(root.is_some());

    Ok(())
}
