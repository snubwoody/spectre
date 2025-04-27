use super::runtime::{EvaluateResponse, ExceptionDetails};
use super::{
    AttachToTargetResponse, CDPMessage, CDPMethod, CDPResponse, GetDocumentResponse,
    GetTargetResponse, PageNavigateResponse, QuerySelectorBody, ResolveNodeBody,
};
use crate::dom::Element;
use crate::{Error, Result, error::CDPError};
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

/// A raw connection to the Chrome Devtool protocol.
#[derive(Debug, Clone)]
pub struct CDPConnection {
    stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl CDPConnection {
    pub async fn new(url: &str) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;
        let stream = Arc::new(Mutex::new(stream));
        Ok(Self { stream })
    }

    pub async fn create_session(mut self) -> Result<CDPSession> {
        let response: GetTargetResponse = self.send(CDPMessage::get_targets(1)).await?;
        let targets = response.body().targets;

        let method = CDPMethod::AttachToTarget {
            target_id: targets[0].target_id.clone(),
            flatten: true,
        };
        let message = CDPMessage::root(1, method);
        let response: AttachToTargetResponse = self.send(message).await?;
        let session_id = response.body().session_id;

        let session = CDPSession::new(self, &session_id);
        Ok(session)
    }

    /// Send a message
    pub async fn send<T>(&self, message: CDPMessage) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let msg: Message = Message::Text(message.json()?.to_string().into());
        let mut stream = self.stream.lock().await;
        stream.send(msg).await?;

        while let Some(Ok(Message::Text(message))) = stream.next().await {
            let json: Value = serde_json::from_str(message.as_str())?;
            // Filter out events and only return responses
            if json.get("id").is_some() {
                match serde_json::from_str(message.as_ref()) {
                    Ok(response) => {
                        return Ok(response);
                    }
                    Err(err) => match serde_json::from_str::<CDPError>(&message) {
                        Ok(cdp_err) => return Err(cdp_err.into()),
                        Err(_) => {
                            let error = Error::InvalidResponse(format!("{}", err));
                            return Err(error);
                        }
                    },
                }
            }
        }

        Err(Error::FailedToSendMessage)
    }
}

#[derive(Debug, Clone)]
pub struct CDPSession {
    conn: CDPConnection,
    session_id: String,
}

impl CDPSession {
    fn new(conn: CDPConnection, session_id: &str) -> Self {
        Self {
            conn,
            session_id: session_id.to_string(),
        }
    }

    /// Navitate the page to a url
    pub async fn navigate(&self, url: &str) -> Result<PageNavigateResponse> {
        self.send(CDPMethod::Navigate {
            url: url.to_string(),
        })
        .await
    }

    /// Get the root DOM node and it's the children upto `depth`. This
    /// method gets all node including non-html elements such as
    /// `#document` and `#text`.
    ///
    /// Set depth to `-1` to get all children.
    ///
    /// # Example
    /// ```
    /// use spectre::{
    ///     Browser,
    ///     Page,
    ///     Result,
    ///     dom::NodeName,
    ///     cdp::CDPConnection
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()>{
    ///     let browser = Browser::start().await?;
    ///     let connection = CDPConnection::new(browser.url()).await?;
    ///     let mut session = connection.create_session().await?;
    ///     
    ///     // Get child nodes up to 5 elements deep;
    ///     let response = session.get_dom(5).await?;
    ///
    ///     let node_name = response.body().root.node_name;
    ///
    ///     assert_eq!(node_name,NodeName::Document);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_dom(&self, depth: i32) -> Result<GetDocumentResponse> {
        self.send(CDPMethod::GetDocument { depth }).await
    }

    /// Resolved the JS node object for a given node id.
    /// The object can than be used in other methods `Runtime.callFunctionOn`
    ///
    /// # Example
    /// ```
    /// use spectre::{Browser,Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()>{
    ///     let mut browser = Browser::start().await?;
    ///     let mut session = browser.get_session().await?;
    ///
    ///     // Resolve the root node
    ///     let response = session.get_dom(-1).await?;
    ///     let root = response.body().root;
    ///     let response = session.resolve_node(root.node_id).await?;
    ///     let object = response.body().object;
    ///     
    ///     assert_eq!(object.description,"#document");
    ///     Ok(())
    /// }
    /// ```
    pub async fn resolve_node(&self, node_id: i32) -> Result<CDPResponse<ResolveNodeBody>> {
        self.send(CDPMethod::ResolveNode { node_id }).await
    }

    /// Run `document.querySelector` on the given node and
    /// return the matched element id;
    ///
    /// # Example
    /// ```
    /// use spectre::{Browser,Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()>{
    ///     let mut browser = Browser::start().await?;
    ///     let mut session = browser.get_session().await?;
    ///
    ///     // Resolve the root node
    ///     let response = session.get_dom(-1).await?;
    ///     let root = response.body().root;
    ///
    ///     // Get the `<body>` element
    ///     let body = session.query_selector(root.node_id,"body").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_selector(&self, node_id: i32, selector: &str) -> Result<Option<Element>> {
        let method = CDPMethod::QuerySelector {
            node_id,
            selector: selector.to_owned(),
        };

        let response: CDPResponse<QuerySelectorBody> = self.send(method).await?;

        // returns 0 if no node is found, so match and return None
        match response.body().node_id {
            0 => Ok(None),
            id => {
                let element = Element::new(id, self.clone());
                Ok(Some(element))
            }
        }
    }

    /// Evaluate javascript string in the browser
    pub async fn evaluate(&self, expr: &str) -> Result<EvaluateResponse> {
        let response: EvaluateResponse = self
            .send(CDPMethod::Evaluate {
                expression: expr.to_string(),
                await_promise: true,
            })
            .await?;

        let body = response.body();

        match body.exception_details {
            Some(details) => {
                let ExceptionDetails {
                    line_number,
                    column_number,
                    ..
                } = details;

                let value = body.result.value;
                let description = body.result.description.unwrap_or_default();

                let error = Error::RuntimeError {
                    line_number,
                    column_number,
                    value,
                    description,
                };

                Err(error)
            }
            None => Ok(response),
        }
    }

    pub async fn send<T>(&self, method: CDPMethod) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id: i32 = rand::random();
        let message = CDPMessage::new(id, &self.session_id, method);
        let response: T = self.conn.send(message).await?;
        Ok(response)
    }
}
