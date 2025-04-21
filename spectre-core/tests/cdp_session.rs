use serde_json::Value;
use spectre_core::{
    Result,
    browser::Browser,
    cdp::{
        AttachToTargetResponse, CDPConnection, CDPMessage, CDPMethod, CreateTargetResponse,
    },
};

#[tokio::test]
async fn create_session() -> Result<()>{
	let browser = Browser::launch().await?;
	let mut connection = CDPConnection::new(browser.url()).await?;
	let session = connection.create_session().await?;

	Ok(())
}