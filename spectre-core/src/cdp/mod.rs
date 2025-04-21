mod connection;
use crate::Result;
use crate::dom::DomNode;
pub use connection::CDPConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CDPMessage {
    id: i32,
    // Null for the root session
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(flatten)]
    method: CDPMethod,
}

impl CDPMessage {
    pub fn new(id: i32, session_id: &str, method: CDPMethod) -> Self {
        Self {
            id,
            session_id: Some(String::from(session_id)),
            method,
        }
    }

    pub fn root(id: i32, method: CDPMethod) -> Self {
        Self {
            id,
            session_id: None,
            method,
        }
    }

    pub fn get_targets(id: i32) -> Self {
        Self {
            id,
            session_id: None,
            method: CDPMethod::GetTargets,
        }
    }

    /// Navigate the page
    ///
    /// Corresponds to [`Page.navigate`](https://vanilla.aslushnikov.com/?Page.navigate)
    pub fn navigate(id: i32, session_id: &str, url: &str) -> Self {
        Self {
            id,
            session_id: Some(String::from(session_id)),
            method: CDPMethod::Navigate {
                url: String::from(url),
            },
        }
    }

    pub fn screenshot(id: i32, session_id: &str, format: ScreenshotFormat) -> Self {
        Self {
            id,
            session_id: Some(String::from(session_id)),
            method: CDPMethod::Screenshot { format },
        }
    }

    /// Evaluate JS in the browser.
    pub fn evaluate(id: i32, session_id: &str, expression: &str) -> Self {
        Self {
            id,
            session_id: Some(session_id.to_string()),
            method: CDPMethod::Evaluate {
                expression: expression.to_string(),
                await_promise: true,
            },
        }
    }

    /// Get the json representation of the message
    pub fn json(&self) -> Result<Value> {
        let json = serde_json::to_value(self)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "method", content = "params")]
pub enum CDPMethod {
    #[serde(rename = "Target.getTargets")]
    GetTargets,
    #[serde(rename = "Target.createTarget")]
    CreateTarget { url: String },
    #[serde(rename = "Target.attachToTarget")]
    #[serde(rename_all = "camelCase")]
    AttachToTarget { target_id: String, flatten: bool },
    #[serde(rename = "Page.captureScreenshot")]
    Screenshot { format: ScreenshotFormat },
    /// Navigates the current page to the given url.
    ///
    /// Corresponds to [`Page.navigate`](https://vanilla.aslushnikov.com/?Page.navigate)
    #[serde(rename = "Page.navigate")]
    Navigate { url: String },
    /// Returns the root DOM node.
    ///
    /// Corresponds to [`DOM.getDocument`](https://vanilla.aslushnikov.com/?DOM.getDocument)
    #[serde(rename = "DOM.getDocument")]
    GetDocument { depth: i32 },
    /// Evaluates expression on global object.
    ///
    /// Corresponds to [`Runtime.evaluate`](https://vanilla.aslushnikov.com/?Runtime.evaluate)
    #[serde(rename = "Runtime.evaluate")]
    #[serde(rename_all = "camelCase")]
    Evaluate {
        /// The expression to evaluate
        expression: String,
        /// Where execution should await the expression and
        /// return the awaited value once the promise is
        /// resolved.
        await_promise: bool,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScreenshotFormat {
    Jpeg,
    Webp,
    Png,
}

pub type GetTargetResponse = CDPResponse<GetTargetBody>;
pub type CreateTargetResponse = CDPResponse<CreateTargetBody>;
pub type AttachToTargetResponse = CDPResponse<AttachToTargetBody>;
pub type PageNavigateResponse = CDPResponse<PageNavigateBody>;
pub type GetDocumentResponse = CDPResponse<GetDocumentBody>;

#[derive(Debug, Deserialize, Serialize)]
pub struct CDPResponse<T> {
    id: i32,
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    result: T,
}

impl<T> CDPResponse<T> {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn body(self) -> T {
        self.result
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetBody {
    pub target_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTargetBody {
    #[serde(rename = "targetInfos")]
    pub targets: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetBody {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentBody {
    pub root: DomNode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageNavigateBody {
    pub frame_id: String,
    pub loader_id: String,
    pub error_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub target_id: String,
    #[serde(rename = "type")]
    target_type: TargetType,
    title: String,
    url: String,
    attached: bool,
    browser_context_id: String,
}

impl Target {
    pub fn id(&self) -> &str {
        &self.target_id
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketTarget {
    pub id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub devtools_frontend_url: String,
    #[serde(rename = "type")]
    pub target_type: TargetType,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    Tab,
    Page,
    Iframe,
    Worker,
    ServiceWorker,
    Browser,
    WebView,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::Browser;

    #[tokio::test]
    async fn send_cdp_message() -> Result<()> {
        let browser = Browser::launch().await?;
        let ws_url = browser.url();

        let mut conn = CDPConnection::new(ws_url).await?;
        let message = CDPMessage::root(2, CDPMethod::GetTargets);
        let _: GetTargetResponse = conn.send(message).await?;

        Ok(())
    }

    #[tokio::test]
    async fn multiple_connections() -> Result<()> {
        let browser = Browser::launch().await?;
        let ws_url = browser.url();

        let mut conn1 = CDPConnection::new(ws_url).await?;
        let mut conn2 = CDPConnection::new(ws_url).await?;

        let _: GetTargetResponse = conn1
            .send(CDPMessage::root(2, CDPMethod::GetTargets))
            .await?;
        let _: GetTargetResponse = conn2
            .send(CDPMessage::root(2, CDPMethod::GetTargets))
            .await?;

        Ok(())
    }
}
