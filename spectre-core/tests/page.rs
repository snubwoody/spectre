use spectre_core::browser::Browser;
use spectre_core::Result;
use spectre_core::page::Page;

#[tokio::test]
async fn navigate_page() -> Result<()>{
	let mut browser = Browser::launch().await?;
	let mut page = browser.goto("https://google.com").await?;
	page.navigate().await?;

	Ok(())
}