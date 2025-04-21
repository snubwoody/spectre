//! Test that all CDP messages and their
//! responses are parsed correctly
use serde_json::Value;
use spectre_core::{
    Result,
    browser::Browser,
    cdp::{
        AttachToTargetResponse, CDPConnection, CDPMessage, CDPMethod, CreateTargetResponse,
        GetDocumentResponse, GetTargetResponse, PageNavigateResponse,
    },
    dom::NodeName,
};

#[tokio::test]
async fn get_targets() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut conn = CDPConnection::root(browser.url()).await?;
    let message = CDPMessage::root(1, CDPMethod::GetTargets);
    let response: GetTargetResponse = conn.send(message).await?;
    assert_eq!(response.id(), 1);

    Ok(())
}

#[tokio::test]
async fn create_target() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut conn = CDPConnection::root(browser.url()).await?;
    let message = CDPMessage::root(
        1,
        CDPMethod::CreateTarget {
            url: String::from("https://example.com"),
        },
    );
    let _: CreateTargetResponse = conn.send(message).await?;

    Ok(())
}

#[tokio::test]
async fn attach_to_target() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut conn = CDPConnection::root(browser.url()).await?;
    let message = CDPMessage::get_targets(1);

    let response: GetTargetResponse = conn.send(message).await?;
    let targets = response.body().targets;

    let method = CDPMethod::AttachToTarget {
        target_id: targets[0].target_id.clone(),
        flatten: true,
    };
    let message = CDPMessage::root(1, method);
    let _: AttachToTargetResponse = conn.send(message).await?;

    Ok(())
}

#[tokio::test]
async fn page_navigate() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut conn = CDPConnection::root(browser.url()).await?;
    let message = CDPMessage::get_targets(1);

    let response: GetTargetResponse = conn.send(message).await?;
    let targets = response.body().targets;

    let method = CDPMethod::AttachToTarget {
        target_id: targets[0].target_id.clone(),
        flatten: true,
    };
    let message = CDPMessage::root(1, method);
    let response: AttachToTargetResponse = conn.send(message).await?;
    let session_id = response.body().session_id;

    let message = CDPMessage::navigate(2, &session_id, "https://youtube.com");
    let _: PageNavigateResponse = conn.send(message).await?;

    Ok(())
}

#[tokio::test]
async fn get_document() -> Result<()> {
    let mut browser = Browser::launch().await?;
    let page = browser.goto("https://example.com").await?;

    let url = page.endpoint();
    let mut conn = CDPConnection::root(url).await?;

    // Set to -1 to get the whole tree
    let method = CDPMethod::GetDocument { depth: -1 };
    let message = CDPMessage::root(2, method);
    let response: GetDocumentResponse = conn.send(message).await?;

    assert_eq!(response.body().root.node_name, NodeName::Document);

    Ok(())
}

#[tokio::test]
async fn page_navigate_error() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut conn = CDPConnection::root(browser.url()).await?;
    let message = CDPMessage::get_targets(1);

    let response: GetTargetResponse = conn.send(message).await?;
    let targets = response.body().targets;

    let method = CDPMethod::AttachToTarget {
        target_id: targets[0].target_id.clone(),
        flatten: true,
    };
    let message = CDPMessage::root(1, method);
    let response: AttachToTargetResponse = conn.send(message).await?;
    let session_id = response.body().session_id;

    let message = CDPMessage::navigate(2, &session_id, "https://dkksksk.com");
    let response: PageNavigateResponse = conn.send(message).await?;

    assert!(response.body().error_text.is_some());
    Ok(())
}

#[tokio::test]
async fn runtime_evaluate() -> Result<()> {
    let browser = Browser::launch().await?;
    let mut conn = CDPConnection::root(browser.url()).await?;

	let response: GetTargetResponse = conn.send(CDPMessage::get_targets(1)).await?;
    let targets = response.body().targets;

    let method = CDPMethod::AttachToTarget {
        target_id: targets[0].target_id.clone(),
        flatten: true,
    };
    let message = CDPMessage::root(1, method);
    let response: AttachToTargetResponse = conn.send(message).await?;
    let session_id = response.body().session_id;

	let method = CDPMethod::Evaluate { 
		expression: String::from("
			function hi(){
				return 4;
			}

			hi()
		"), 
		await_promise: true 
	};

	let message = CDPMessage::new(1,&session_id, method);
    let response: Value = conn.send(message).await?;
	dbg!(&response);

	Ok(())
}
