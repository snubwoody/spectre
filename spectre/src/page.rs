use crate::{
    Result,
    cdp::{CDPConnection, CDPMessage, ScreenshotFormat},
};
use serde_json::Value;

#[derive(Debug)]
pub struct Page {
    session_id: String,
    conn: CDPConnection,
}

impl Page {
    pub async fn new(session_id: &str, url: &str) -> Result<Self> {
        let conn = CDPConnection::new(url, Some(session_id.to_string())).await?;
        Ok(Page {
            session_id: String::from(session_id),
            conn,
        })
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub async fn screenshot(&mut self) -> Result<()> {
        let message = CDPMessage::screenshot(1, &self.session_id, ScreenshotFormat::Png);

        let response = self.conn.send::<Value>(message).await;
        dbg!(response);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::Browser;

    #[tokio::test]
    pub async fn capture_screenshot() -> Result<()> {
        let mut browser = Browser::launch().await?;
        let mut page = browser.goto("https://google.com").await?;
        let targets = browser.get_targets().await?;
        // dbg!(&targets);
        page.screenshot().await?;

        Ok(())
    }
}
