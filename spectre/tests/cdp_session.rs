use serde_json::json;
use spectre::{
    Browser, EMPTY_PAGE, Error, Result,
    cdp::{CDPConnection, CDPMethod, GetTargetResponse},
};

#[tokio::test]
async fn create_session() -> Result<()> {
    let browser = Browser::launch().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

    let _: GetTargetResponse = session.send(CDPMethod::GetTargets).await?;
    Ok(())
}

#[tokio::test]
async fn navigate() -> Result<()> {
    let browser = Browser::launch().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

    session.navigate(EMPTY_PAGE).await?;
    Ok(())
}

#[tokio::test]
async fn evaluate() -> Result<()> {
    let browser = Browser::launch().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

    let _ = session.evaluate("5").await?;
    Ok(())
}

#[tokio::test]
async fn can_handle_exception() -> Result<()> {
    let browser = Browser::launch().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

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
    let browser = Browser::launch().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;

    let result = session.evaluate("return 5").await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn dom_node_into_element() -> Result<()> {
    let browser = Browser::launch().await?;
    let connection = CDPConnection::new(browser.url()).await?;
    let mut session = connection.create_session().await?;
    let response = session.get_dom(-1).await?;
    let root = response.body().root;
    let element = root.into_element(session);
    dbg!(element);
    Ok(())
}
