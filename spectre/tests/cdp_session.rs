use serde_json::json;
use spectre::{
    EMPTY_PAGE, Error, Result,
    cdp::{CDPConnection, CDPMethod, GetTargetResponse},
};

#[spectre::test]
async fn create_session() -> Result<()> {
    let connection = CDPConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let _: GetTargetResponse = session.send(CDPMethod::GetTargets).await?;
    Ok(())
}

#[spectre::test]
async fn get_box_model() -> Result<()> {
    let connection = CDPConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let root = session.get_dom(-1).await?;
    session.get_box_model(root.node_id).await?;

    Ok(())
}

#[spectre::test]
async fn navigate() -> Result<()> {
    let connection = CDPConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    session.navigate(EMPTY_PAGE).await?;
    Ok(())
}

#[spectre::test]
async fn evaluate() -> Result<()> {
    let connection = CDPConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let _ = session.evaluate("5").await?;
    Ok(())
}

#[spectre::test]
async fn can_handle_exception() -> Result<()> {
    let connection = CDPConnection::new(&browser.url()).await?;
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

#[spectre::test]
async fn can_handle_syntax_error() -> Result<()> {
    let connection = CDPConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let result = session.evaluate("return 5").await;
    assert!(result.is_err());
    Ok(())
}

#[spectre::test]
async fn query_selector_handle_missing_element() -> Result<()> {
    let connection = CDPConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let root = session.get_dom(-1).await?;

    let element = session
        .query_selector(root.node_id, ".very-unique-class")
        .await?;
    assert!(element.is_none());

    let expr = "
		let element = document.createElement('div');
		element.className = 'very-unique-class';
		document.body.appendChild(element);
	";

    session.evaluate(expr).await?;
    let element = session
        .query_selector(root.node_id, ".very-unique-class")
        .await?;
    assert!(element.is_some());

    Ok(())
}
