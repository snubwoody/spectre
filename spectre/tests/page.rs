use spectre::Browser;
use spectre::dom::NodeName;
use spectre::{EMPTY_PAGE, Result};

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
