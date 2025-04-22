use crate::{
    cdp::{
        CDPConnection, CDPMessage, CDPMethod, CDPSession, GetDocumentResponse, PageNavigateResponse, ScreenshotFormat
    }, dom::{DomNode, NodeName}, Result
};
use serde_json::Value;

// TODO impl Drop

#[derive(Debug)]
pub struct Page {
    session: CDPSession,
    endpoint: String,
}

impl Page {
    pub async fn new(session_id: &str, url: &str) -> Result<Self> {
        let mut conn = CDPConnection::new(url).await?;
		let session = conn.create_session().await?;
		
        Ok(Page {
            session,
            endpoint: String::from(url),
        })
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    async fn get_dom(&mut self) -> Result<DomNode> {
        // Set to -1 to get all sub nodes.
        let method = CDPMethod::GetDocument { depth: -1 };
        let response: GetDocumentResponse = self.session.send(method).await?;
        let root = response.body().root;

        Ok(root)
    }

    /// Get an element by it's name.
    pub async fn get_by_name(&mut self, name: NodeName) -> Result<Option<DomNode>> {
        let root = self.get_dom().await?;
        Ok(root.get_by_name(&name))
    }

    pub async fn navigate(&mut self) -> Result<()> {
        let method = CDPMethod::Navigate {
            url: String::from("https://youtube.com"),
        };
        let response: PageNavigateResponse = self.session.send(method).await?;
        Ok(())
    }

}