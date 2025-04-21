use super::runtime::{EvaluateResponse, ExceptionDetails};
use super::{
    AttachToTargetResponse, CDPMessage, CDPMethod, GetTargetResponse, PageNavigateResponse,
};
use crate::{Error, Result, error::CDPError};
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

/// A raw connection to the Chrome Devtool protocol.
#[derive(Debug)]
pub struct CDPConnection {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl CDPConnection {
    pub async fn new(url: &str) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;
        Ok(Self { stream })
    }

    pub async fn create_session(&mut self) -> Result<CDPSession> {
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

    /// Connect to the root web socket i.e the browser
    pub async fn root(url: &str) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;
        Ok(Self { stream })
    }

    /// Send a message
    pub async fn send<T>(&mut self, message: CDPMessage) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let msg: Message = Message::Text(message.json()?.to_string().into());
        self.stream.send(msg).await?;

        while let Some(Ok(Message::Text(message))) = self.stream.next().await {
            let json: Value = serde_json::from_str(message.as_str())?;
            // Filter out events and only return
            // responses
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

#[derive(Debug)]
pub struct CDPSession<'conn> {
    conn: &'conn mut CDPConnection,
    session_id: String,
}

impl<'conn> CDPSession<'conn> {
    fn new(conn: &'conn mut CDPConnection, session_id: &str) -> Self {
        Self {
            conn,
            session_id: session_id.to_string(),
        }
    }

    /// Navitate the page to a url
    pub async fn navigate(&mut self, url: &str) -> Result<PageNavigateResponse> {
        self.send(CDPMethod::Navigate {
            url: url.to_string(),
        })
        .await
    }

    /// Evaluate javascript in the browser
    pub async fn evaluate(&mut self, expr: &str) -> Result<EvaluateResponse> {
        let response: EvaluateResponse = self.send(CDPMethod::Evaluate {
            expression: expr.to_string(),
            await_promise: true,
        })
        .await?;

		let body = response.body();

		match body.exception_details {
			Some(details) => {
				let ExceptionDetails{
					line_number,
					column_number,
					..
				} = details;

				let value = body.result.value;
				let description = body.result.description;

				let error = Error::RuntimeError { 
					line_number, 
					column_number, 
					value, 
					description
				};

				Err(error)
			},
			None => return  Ok(response)
		}
    }

    pub async fn send<T>(&mut self, method: CDPMethod) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id: i32 = rand::random();
        let message = CDPMessage::new(id, &self.session_id, method);
        let response: T = self.conn.send(message).await?;
        Ok(response)
    }
}
