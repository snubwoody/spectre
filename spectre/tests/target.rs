use spectre::{
    Result,
    cdp::{CdpConnection, CDPMessage, CdpMethod, GetTargetResponse},
};

#[spectre::test]
async fn get_targets() -> Result<()> {
    let conn = CdpConnection::new(&browser.url()).await?;
    let message = CDPMessage::root(1, CdpMethod::GetTargets);
    let response: GetTargetResponse = conn.send(message).await?;

    assert_eq!(response.id(), 1);
    Ok(())
}
