//! Test that all CDP messages and their
//! responses are parsed correctly
use serde_json::Value;
use spectre::{
    Result,
    cdp::{
        AttachToTargetResponse, CdpConnection, CDPMessage, CdpMethod, CreateTargetResponse,
        GetDocumentResponse, GetTargetResponse, PageNavigateResponse,
    },
    dom::NodeName,
};

#[spectre::test]
async fn get_targets() -> Result<()> {
    let conn = CdpConnection::new(&browser.url()).await?;
    let message = CDPMessage::root(1, CdpMethod::GetTargets);
    let response: GetTargetResponse = conn.send(message).await?;
    assert_eq!(response.id(), 1);

    Ok(())
}

#[spectre::test]
async fn create_target() -> Result<()> {
    let conn = CdpConnection::new(&browser.url()).await?;
    let message = CDPMessage::root(
        1,
        CdpMethod::CreateTarget {
            url: String::from("https://example.com"),
        },
    );
    let _: CreateTargetResponse = conn.send(message).await?;

    Ok(())
}

#[spectre::test]
async fn attach_to_target() -> Result<()> {
    let conn = CdpConnection::new(&browser.url()).await?;
    let message = CDPMessage::get_targets(1);

    let response: GetTargetResponse = conn.send(message).await?;
    let targets = response.body().targets;

    let method = CdpMethod::AttachToTarget {
        target_id: targets[0].target_id.clone(),
        flatten: true,
    };
    let message = CDPMessage::root(1, method);
    let _: AttachToTargetResponse = conn.send(message).await?;

    Ok(())
}

#[spectre::test]
async fn page_navigate() -> Result<()> {
    let conn = CdpConnection::new(&browser.url()).await?;
    let message = CDPMessage::get_targets(1);

    let response: GetTargetResponse = conn.send(message).await?;
    let targets = response.body().targets;

    let method = CdpMethod::AttachToTarget {
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

#[spectre::test]
async fn get_document() -> Result<()> {
    let page = browser.goto("https://example.com").await?;

    let url = page.endpoint();
    let conn = CdpConnection::new(url).await?;

    // Set to -1 to get the whole tree
    let method = CdpMethod::GetDocument { depth: -1 };
    let message = CDPMessage::root(2, method);
    let response: GetDocumentResponse = conn.send(message).await?;

    assert_eq!(response.body().root.node_name, NodeName::Document);

    Ok(())
}

#[spectre::test]
async fn page_navigate_error() -> Result<()> {
    let conn = CdpConnection::new(&browser.url()).await?;
    let message = CDPMessage::get_targets(1);

    let response: GetTargetResponse = conn.send(message).await?;
    let targets = response.body().targets;

    let method = CdpMethod::AttachToTarget {
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

#[spectre::test]
async fn runtime_evaluate() -> Result<()> {
    let conn = CdpConnection::new(&browser.url()).await?;

    let response: GetTargetResponse = conn.send(CDPMessage::get_targets(1)).await?;
    let targets = response.body().targets;

    let method = CdpMethod::AttachToTarget {
        target_id: targets[0].target_id.clone(),
        flatten: true,
    };
    let message = CDPMessage::root(1, method);
    let response: AttachToTargetResponse = conn.send(message).await?;
    let session_id = response.body().session_id;

    let method = CdpMethod::Evaluate {
        expression: String::from(
            "
			function hi(){
				return 4;
			}

			hi()
		",
        ),
        await_promise: true,
    };

    let message = CDPMessage::new(1, &session_id, method);
    let response: Value = conn.send(message).await?;
    dbg!(&response);

    Ok(())
}

#[spectre::test]
async fn send_cdp_message() -> Result<()> {
    let ws_url = browser.url();

    let conn = CdpConnection::new(&ws_url).await?;
    let message = CDPMessage::root(2, CdpMethod::GetTargets);
    let _: GetTargetResponse = conn.send(message).await?;

    Ok(())
}

#[spectre::test]
async fn multiple_connections() -> Result<()> {
    let ws_url = browser.url();

    let conn1 = CdpConnection::new(&ws_url).await?;
    let conn2 = CdpConnection::new(&ws_url).await?;

    let _: GetTargetResponse = conn1
        .send(CDPMessage::root(2, CdpMethod::GetTargets))
        .await?;
    let _: GetTargetResponse = conn2
        .send(CDPMessage::root(2, CdpMethod::GetTargets))
        .await?;

    Ok(())
}
