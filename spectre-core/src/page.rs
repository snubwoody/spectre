use crate::{
    Result,
    cdp::{
        CDPConnection, CDPMessage, CDPMethod, GetDocumentResponse, PageNavigateResponse,
        ScreenshotFormat,
    },
    dom::DomNode,
};
use serde_json::Value;

// TODO impl Drop

#[derive(Debug)]
pub struct Page {
    session_id: String,
    conn: CDPConnection,
    endpoint: String,
}

impl Page {
    pub async fn new(session_id: &str, url: &str) -> Result<Self> {
        let conn = CDPConnection::new(url, Some(session_id.to_string())).await?;
        Ok(Page {
            session_id: String::from(session_id),
            conn,
            endpoint: String::from(url),
        })
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    async fn get_dom(&mut self) -> Result<DomNode> {
        // Set to -1 to get all sub nodes.
        let method = CDPMethod::GetDocument { depth: -1 };
        let message = CDPMessage::root(1, method);
        let response: GetDocumentResponse = self.conn.send(message).await?;

        Ok(response.body().root)
    }

    pub async fn get_by_name(&mut self) -> Result<()> {
        let root = self.get_dom().await?;
        dbg!(root);

        Ok(())
    }

    pub async fn navigate(&mut self) -> Result<()> {
        let method = CDPMethod::Navigate {
            url: String::from("https://youtube.com"),
        };
        let message = CDPMessage::root(2, method);
        dbg!(&message);
        let response: PageNavigateResponse = self.conn.send(message).await?;
        dbg!(response);
        Ok(())
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
