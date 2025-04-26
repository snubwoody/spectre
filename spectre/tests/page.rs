use spectre::{Result, EMPTY_PAGE};
use spectre::Browser;
use spectre::dom::NodeName;

#[tokio::test]
async fn get_by_class() -> Result<()>{ 
	let mut browser = Browser::launch().await?;
    let mut page = browser.new_page().await?;
	page.navigate(EMPTY_PAGE).await?;

	let expr = "
		let element = document.createElement('button');
		element.className = 'btn btn-primary btn-large';
		document.body.appendChild(element);
	";

	page.evaluate(expr).await?;

	let button = page.get_by_class("btn").await?;
	Ok(())
}

#[tokio::test]
async fn call_function_on_element() -> Result<()>{ 
	let mut browser = Browser::launch().await?;
    let mut page = browser.new_page().await?;
	page.navigate(EMPTY_PAGE).await?;

	let expr = "
		let element = document.createElement('button');
		element.className = 'btn btn-primary btn-large';
		document.body.appendChild(element);
	";

	page.evaluate(expr).await?;
	let root = page.get_dom().await?;
	let id = &root.node_id;
	
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
