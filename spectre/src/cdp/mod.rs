//! Manages sending and receiving messages to the browser via
//! the Chrome DevTools Protocol.
mod connection;
pub mod runtime;
use crate::Result;
use crate::dom::DomNode;
pub use connection::{CDPConnection, CDPSession};
use runtime::RemoteObject;
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

    /// Get the json representation of the message
    pub fn json(&self) -> Result<Value> {
        let json = serde_json::to_value(self)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
    /// Compiles an expression into a script.
    ///
    /// Corresponds to [`Runtime.compileScript`](https://vanilla.aslushnikov.com/?Dom.resolveNode)
    #[serde(rename = "DOM.resolveNode")]
    #[serde(rename_all = "camelCase")]
    ResolveNode { node_id: i32 },
    /// Runs `document.querySelector` on the given node. See the
    /// [mdn docs](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)
    /// for more details.
    ///
    /// Corresponds to [`DOM.querySelector`](https://vanilla.aslushnikov.com/?DOM.querySelector)
    #[serde(rename = "DOM.querySelector")]
    #[serde(rename_all = "camelCase")]
    QuerySelector { node_id: i32, selector: String },
    /// Get the box model for the given node.
    ///
    /// Corresponds to [`DOM.querySelector`](https://vanilla.aslushnikov.com/?DOM.getBoxModel)
    #[serde(rename = "DOM.getBoxModel")]
    #[serde(rename_all = "camelCase")]
    GetBoxModel { node_id: i32 },
}

/// Screenshot image formats supported by the browser
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
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
pub type ResolveNodeResponse = CDPResponse<ResolveNodeBody>;

#[derive(Debug, Deserialize, Serialize)]
pub struct CDPResponse<T: Clone> {
    id: i32,
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    result: T,
}

impl<T> CDPResponse<T>
where
    T: Clone,
{
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn body(&self) -> T {
        self.result.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResolveNodeBody {
    pub object: RemoteObject,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetBody {
    pub target_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTargetBody {
    #[serde(rename = "targetInfos")]
    pub targets: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetBody {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetDocumentBody {
    pub root: DomNode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorBody {
    pub node_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        let browser = Browser::start().await?;
        let ws_url = browser.url();

        let conn = CDPConnection::new(&ws_url).await?;
        let message = CDPMessage::root(2, CDPMethod::GetTargets);
        let _: GetTargetResponse = conn.send(message).await?;

        Ok(())
    }

    #[tokio::test]
    async fn multiple_connections() -> Result<()> {
        let browser = Browser::start().await?;
        let ws_url = browser.url();

        let conn1 = CDPConnection::new(&ws_url).await?;
        let conn2 = CDPConnection::new(&ws_url).await?;

        let _: GetTargetResponse = conn1
            .send(CDPMessage::root(2, CDPMethod::GetTargets))
            .await?;
        let _: GetTargetResponse = conn2
            .send(CDPMessage::root(2, CDPMethod::GetTargets))
            .await?;

        Ok(())
    }
}
