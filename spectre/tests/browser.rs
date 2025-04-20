use std::time::Duration;

use spectre::{Browser,Result};

#[tokio::test]
async fn create_page() -> Result<()>{
	let mut browser = Browser::launch().await?;
	let mut page = browser.goto("https://youtube.com").await?;
	let targets = browser.get_targets().await?;
	page.navigate().await?;
	dbg!(&page.session_id());
	// dbg!(&targets);
	Ok(())
}