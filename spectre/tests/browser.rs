use spectre::{Browser,Result};

#[tokio::test]
async fn create_page() -> Result<()>{
	let mut browser = Browser::launch().await?;
	let page = browser.goto("https://youtube.com").await?;
	let targets = browser.get_targets().await?;
	dbg!(&page);
	dbg!(&targets);
	Ok(())
}