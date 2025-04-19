//! Test that all CDP messages and their 
//! responses are parsed correctly
use serde_json::Value;
use spectre::{
	browser::Browser, 
	cdp::{CDPConnection, CDPMessage, CDPMethod, GetTargetResponse}, 
	Result
};

#[tokio::test]
async fn get_targets() -> Result<()>{
	let browser = Browser::launch().await?;
	let mut conn = CDPConnection::root(browser.url()).await?;
	let message = CDPMessage::root(1, CDPMethod::GetTargets);
	let response: GetTargetResponse = conn.send(message).await?;
	assert_eq!(response.id,1);
	
	Ok(())
}

#[tokio::test]
async fn create_target() -> Result<()>{
	let browser = Browser::launch().await?;
	let mut conn = CDPConnection::root(browser.url()).await?;
	let message = CDPMessage::root(1, CDPMethod::GetTargets);
	let response: GetTargetResponse = conn.send(message).await?;
	assert_eq!(response.id,1);

	Ok(())
}
