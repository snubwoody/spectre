use spectre_core::dom::NodeName;
use spectre_core::Result;
use spectre_core::browser::Browser;

#[tokio::test]
async fn get_by_name() -> Result<()> {
    let mut browser = Browser::launch().await?;
    let mut page = browser.goto("https://google.com").await?;
    
	let root = page.get_by_name(NodeName::Document).await?;
	assert!(root.is_some());

    Ok(())
}
