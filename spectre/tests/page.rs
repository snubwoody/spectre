use spectre::browser::Browser;
use spectre::Result;
use spectre::page::Page;

#[tokio::test]
async fn navigate_page() -> Result<()>{
	let mut browser = Browser::launch().await?;
	let mut page = browser.goto("https://google.com").await?;
	page.navigate().await?;

	Ok(())
}