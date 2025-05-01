use spectre::{
    cdp::{
        CDPConnection, CDPMessage, CDPMethod,
        GetTargetResponse, 
    }, Result
};

#[spectre::test]
async fn get_targets() -> Result<()> {
    let conn = CDPConnection::new(&browser.url()).await?;
    let message = CDPMessage::root(1, CDPMethod::GetTargets);
    let response: GetTargetResponse = conn.send(message).await?;

    assert_eq!(response.id(), 1);
    Ok(())
}

