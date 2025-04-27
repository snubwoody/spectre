use serde_json::json;
use spectre::{
    Browser, EMPTY_PAGE, Error, Result,
    cdp::{CDPConnection, CDPMethod, GetTargetResponse},
};

#[tokio::test]
async fn create_session() -> Result<()> {
    let browser = Browser::start().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let session = connection.create_session().await?;

    let _: GetTargetResponse = session.send(CDPMethod::GetTargets).await?;
    Ok(())
}

#[tokio::test]
async fn navigate() -> Result<()> {
    let browser = Browser::start().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let session = connection.create_session().await?;

    session.navigate(EMPTY_PAGE).await?;
    Ok(())
}

#[tokio::test]
async fn evaluate() -> Result<()> {
    let browser = Browser::start().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let session = connection.create_session().await?;

    let _ = session.evaluate("5").await?;
    Ok(())
}

#[tokio::test]
async fn can_handle_exception() -> Result<()> {
    let browser = Browser::start().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let session = connection.create_session().await?;

    let result = session.evaluate("throw 5").await;
    let error = result.err().unwrap();

    let num = json!(5);

    match error {
        Error::RuntimeError { value, .. } => {
            assert_eq!(value, Some(num))
        }
        _ => {
            panic!("Invalid error type")
        }
    }

    Ok(())
}

#[tokio::test]
async fn can_handle_syntax_error() -> Result<()> {
    let browser = Browser::start().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let session = connection.create_session().await?;

    let result = session.evaluate("return 5").await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn query_selector_handle_missing_element() -> Result<()> {
    let browser = Browser::start().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let session = connection.create_session().await?;

    let response = session.get_dom(-1).await?;
	let root = response.body().root;
	
	let element = session.query_selector(root.node_id, ".very-unique-class").await?;
	assert!(element.is_none());
	
	let expr = "
		let element = document.createElement('div');
		element.className = 'very-unique-class';
		document.body.appendChild(element);
	";
		
	session.evaluate(expr).await?;
	let element = session.query_selector(root.node_id, ".very-unique-class").await?;
	assert!(element.is_some());

	Ok(())
}
