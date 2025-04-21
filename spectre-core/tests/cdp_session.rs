use spectre_core::{browser::Browser, cdp::{runtime::EvaluateResponse, CDPConnection, CDPMethod, GetTargetResponse}, Result, EMPTY_PAGE};

#[tokio::test]
async fn create_session() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

	let _: GetTargetResponse = session.send(CDPMethod::GetTargets).await?;
	Ok(())
}

#[tokio::test]
async fn navigate() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

	session.navigate(EMPTY_PAGE).await?;
	Ok(())
}

#[tokio::test]
async fn evaluate() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

	let response = session.evaluate("5").await?;
	dbg!(response);

	Ok(())
}

#[tokio::test]
async fn evaluate_error() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

	let response = session.evaluate("return 5").await?;
	dbg!(response);

	Ok(())
}
