use spectre_core::browser::Browser;
use spectre_core::Result;

#[tokio::test]
async fn get_by_name() -> Result<()>{
	let mut browser = Browser::launch().await?;
	let mut page = browser.goto("https://google.com").await?;
	page.get_by_name().await?;

	Ok(())
}