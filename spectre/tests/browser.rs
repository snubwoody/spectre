use spectre::{Browser, get_available_port};
use std::mem;
use tokio_tungstenite::connect_async;

#[spectre::test]
async fn goto_page() -> spectre::Result<()> {
    let _ = browser.goto("https://youtube.com").await?;
    let targets = browser.get_targets().await?;
    targets
        .iter()
        .find(|t| t.url() == "https://www.youtube.com/")
        .unwrap();

    Ok(())
}

#[tokio::test]
async fn connect_to_running_browser() -> spectre::Result<()> {
    let port = get_available_port().await?;
    let _browser = Browser::start_on(port).await?;
    Browser::connect(port).await?;

    Ok(())
}

#[tokio::test]
async fn check_if_browser_is_running() -> spectre::Result<()> {
    let port = get_available_port().await?;
    assert!(!Browser::is_running(port).await);
    let _browser = Browser::start_on(port).await?;

    assert!(Browser::is_running(port).await);

    Ok(())
}

#[tokio::test]
async fn browser_closed_when_dropped() -> spectre::Result<()> {
    let browser = Browser::start().await?;
    let url = browser.url();

    let _ = connect_async(&url).await?;
    mem::drop(browser);

    let result = connect_async(url).await;
    assert!(result.is_err());
    Ok(())
}
