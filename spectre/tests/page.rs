use spectre::Browser;
use spectre::dom::NodeName;
use spectre::{EMPTY_PAGE, Result};

#[tokio::test]
async fn get_by_name() -> Result<()> {
    let mut browser = Browser::start().await?;
    let mut page = browser.goto("https://google.com").await?;

    let root = page.get_by_name(NodeName::Document).await?;
    assert!(root.is_some());

    Ok(())
}

#[tokio::test]
async fn get_url() -> Result<()> {
    let mut browser = Browser::start().await?;
    let mut page = browser.goto("https://google.com").await?;

    let root = page.get_by_name(NodeName::Document).await?;
    assert!(root.is_some());

    Ok(())
}

#[tokio::test]
async fn get_by_class() -> Result<()> {
    let mut browser = Browser::start().await?;
    let mut page = browser.new_page().await?;
	let expr = "
		document.createElement()
	";

    let root = page.get_dom().await?;

    Ok(())
}
