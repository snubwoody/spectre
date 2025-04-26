use spectre_core::{Result, EMPTY_PAGE};
use spectre_core::browser::Browser;
use spectre_core::dom::NodeName;

#[tokio::test]
async fn get_by_class() -> Result<()>{ 
	let mut browser = Browser::launch().await?;
    let mut page = browser.new_page().await?;
	page.navigate("https://blank.org/").await?;

	let expr = "
		let element = document.createElement('button');
		element.className = 'btn';
		document.body.appendChild(element);
	";

	page.evaluate(expr).await?;

	let button = page.get_by_class("btn").await?;
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
