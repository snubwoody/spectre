use spectre_core::{
    Result,
    browser::Browser,
    cdp::CDPConnection,
};

#[tokio::test]
async fn create_session() -> Result<()>{
	let browser = Browser::launch().await?;
	let mut connection = CDPConnection::new(browser.url()).await?;
	let _ = connection.create_session().await?;

	Ok(())
}