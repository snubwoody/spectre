use spectre_core::{Result, EMPTY_PAGE};
use spectre_core::browser::Browser;
use spectre_core::dom::NodeName;

#[tokio::test]
async fn get_dom() -> Result<()>{ 
	let mut browser = Browser::launch().await?;
    let mut page = browser.goto(EMPTY_PAGE).await?;

    let root = page.get_dom().await?;
	dbg!(&root);
	
	Ok(())
}

#[tokio::test]
async fn get_by_name() -> Result<()> {
    let mut browser = Browser::launch().await?;
    let mut page = browser.goto("https://google.com").await?;

    let root = page.get_by_name(NodeName::Document).await?;
    assert!(root.is_some());

    Ok(())
}
