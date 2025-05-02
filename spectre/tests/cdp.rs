use serde_json::{Value, json};
use spectre::{
    Browser, EMPTY_PAGE, Error, Result,
    browser::Cookie,
    cdp::{CdpConnection, CdpMessage, CdpMethod, GetTargetResponse},
};

#[tokio::test]
async fn default_target() -> Result<()> {
    let browser = Browser::start().await?;
    let message = CdpMessage::root(0, CdpMethod::GetTargets);
    let connection = CdpConnection::new(&browser.url()).await?;
    let response: GetTargetResponse = connection.send(message).await?;
    let targets = response.body().targets;
    assert_eq!(targets[0].url(), "chrome://newtab/");

    Ok(())
}

#[tokio::test]
async fn close_target() -> Result<()> {
    let browser = Browser::start().await?;
    let connection = CdpConnection::new(&browser.url()).await?;

    let message = CdpMessage::root(0, CdpMethod::GetTargets);
    let response: GetTargetResponse = connection.send(message).await?;
    let targets = response.body().targets;

    assert!(targets.len() == 1);

    let target_id = targets[0].target_id.clone();
    let message = CdpMessage::root(1, CdpMethod::CloseTarget { target_id });

    connection.send::<Value>(message).await?;

    let message = CdpMessage::root(0, CdpMethod::GetTargets);
    let response: GetTargetResponse = connection.send(message).await?;
    let targets = response.body().targets;
    assert!(targets.is_empty());

    Ok(())
}

#[spectre::test]
async fn create_session() -> Result<()> {
    let connection = CdpConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let _: GetTargetResponse = session.send(CdpMethod::GetTargets).await?;
    Ok(())
}

#[spectre::test]
async fn get_box_model() -> Result<()> {
    let connection = CdpConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let root = session.get_dom(-1).await?;
    session.get_box_model(root.node_id).await?;

    Ok(())
}

#[spectre::test]
async fn navigate() -> Result<()> {
    let connection = CdpConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    session.navigate(EMPTY_PAGE).await?;
    Ok(())
}

#[tokio::test]
async fn set_cookies() -> Result<()> {
    let browser = Browser::start().await?;
    let cookie = Cookie::default();
    let name = cookie.name.clone();
    let cookies = vec![cookie];

    let connection = CdpConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;
    session.set_cookies(cookies).await?;

    let cookies = session.get_cookies().await?;
    cookies
        .iter()
        .find(|c| c.name == name)
        .expect("Cookie not set");
    Ok(())
}

#[spectre::test]
async fn evaluate() -> Result<()> {
    let connection = CdpConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let _ = session.evaluate("5").await?;
    Ok(())
}

#[spectre::test]
async fn can_handle_exception() -> Result<()> {
    let connection = CdpConnection::new(&browser.url()).await?;
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
    let connection = CdpConnection::new(&browser.url()).await?;
    let session = connection.create_session().await?;

    let result = session.evaluate("return 5").await;
    assert!(result.is_err());
    Ok(())
}

#[spectre::test]
async fn query_selector_handle_missing_element() -> Result<()> {
    let connection = CdpConnection::new(&browser.url()).await?;
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
