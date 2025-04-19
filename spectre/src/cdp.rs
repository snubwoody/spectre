use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::{error::CDPError, Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;


#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all="camelCase")]
pub struct CDPMessage{
	id: i32,
	// Null for the root session
	#[serde(skip_serializing_if="Option::is_none")]
	session_id: Option<String>,
	#[serde(flatten)]
	method: CDPMethod
}

impl CDPMessage{
	pub fn new(id:i32,session_id: &str,method: CDPMethod) -> Self{
		Self { id, session_id: Some(String::from(session_id)), method }
	}

	pub fn root(id:i32,method: CDPMethod) -> Self{
		Self{
			id,
			session_id: None,
			method
		}
	}

	pub fn get_targets(id:i32,session_id: &str) -> Self{
		Self { 
			id, 
			session_id: Some(String::from(session_id)), 
			method: CDPMethod::GetTargets
		}
	}

	pub fn screenshot(id:i32,session_id: &str,format: ScreenshotFormat) -> Self{
		Self { 
			id, 
			session_id: Some(String::from(session_id)), 
			method: CDPMethod::Screenshot{ format}
		}
	}

	/// Get the json representation of the message
	pub fn json(&self) -> Result<Value>{
		let json = serde_json::to_value(self)?;
		Ok(json)
	}
}



#[derive(Debug,Deserialize,Serialize)]
#[serde(tag="method",content="params")]
pub enum CDPMethod{
	#[serde(rename="Target.getTargets")]
	GetTargets,
	#[serde(rename="Target.createTarget")]
	CreateTarget{
		url: String
	},
	#[serde(rename="Target.attachToTarget")]
	#[serde(rename_all="camelCase")]
	AttachToTarget{
		target_id: String,
		flatten: bool
	},
	#[serde(rename="Page.captureScreenshot")]
	Screenshot{
		format: ScreenshotFormat
	}
}

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all="lowercase")]
pub enum ScreenshotFormat{
	Jpeg,
	Webp,
	Png,
}

pub type GetTargetResponse = CDPResponse<GetTargetBody>;
pub type CreateTargetResponse = CDPResponse<CreateTargetBody>;

#[derive(Debug,Deserialize,Serialize)]
pub struct CDPResponse<T>{
	id: i32,
	result: T
}

impl<T> CDPResponse<T>{
	pub fn id(&self) -> i32{
		self.id
	}

	pub fn body(self) -> T{
		self.result
	}
}


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateTargetBody{
	pub target_id: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct GetTargetBody{
	#[serde(rename="targetInfos")]
	pub targets: Vec<Target>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AttachToTargetResponse{
	method: String,
	params: AttachToTargetBody
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct AttachToTargetBody{
	session_id: String,
	target_info: Target,
	waiting_for_debugger:bool
}

impl AttachToTargetResponse{
	pub fn session_id(&self) -> &str{
		&self.params.session_id
	}
}

#[derive(Debug,Serialize,Deserialize,Clone)]
#[serde(rename_all="camelCase")]
pub struct Target{
	target_id: String,
	#[serde(rename="type")]
	target_type: TargetType,
	title: String,
	url: String,
	attached: bool,
	browser_context_id: String
}

impl Target{
	pub fn id(&self) -> &str{
		&self.target_id
	}

	pub fn url(&self) -> &str{
		&self.url
	}
}

#[derive(Debug,Serialize,Deserialize,Clone, Copy,PartialEq)]
#[serde(rename_all="snake_case")]
pub enum TargetType{
	Tab,
	Page,
	Iframe,
	Worker,
	ServiceWorker,
	Browser,
	WebView
}

#[derive(Debug)]
pub struct CDPConnection{
	stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
	// Empty for the root session
	session_id: Option<String> 
}

impl CDPConnection{
	pub async fn new(url: &str,session_id: Option<String>) -> Result<Self>{
		let (stream, _) = connect_async(url).await?;
		Ok(Self { stream,session_id })
	}

	/// Connect to the root web socket i.e the browser
	pub async fn root(url: &str) -> Result<Self>{
		let (stream, _) = connect_async(url).await?;
		Ok(Self { stream,session_id:None })
	}

	/// Send a message 
	pub async fn send<T>(&mut self,message: CDPMessage) -> Result<T>
	where T: DeserializeOwned + Debug
	{
		let msg: Message = Message::Text(message.json()?.to_string().into());
		self.stream.send(msg).await?;

		if let Some(Ok(Message::Text(message))) = self.stream.next().await{
			match serde_json::from_str(&message.to_string()) {
				Ok(response) => {
					return Ok(response);
				}
				Err(err)=>{
					dbg!(&message);
					dbg!(&err);
					let response: CDPError = serde_json::from_str(&message)?;
					return Err(response.into());
				}
			}
		}

		Err(Error::FailedToSendMessage)
	}
}

#[cfg(test)]
mod tests{
	use crate::browser::Browser;
	use super::*;

	#[tokio::test]
	async fn send_cdp_message() -> Result<()>{
		let browser = Browser::launch().await?;
		let ws_url = browser.url();

		let mut conn = CDPConnection::new(ws_url, None).await?;
		let message = CDPMessage::root(2, CDPMethod::GetTargets);
		let _: GetTargetResponse = conn.send(message).await?;

		Ok(())
	}

	#[tokio::test]
	async fn multiple_connections() -> Result<()>{
		let browser = Browser::launch().await?;
		let ws_url = browser.url();

		let mut conn1 = CDPConnection::new(ws_url, None).await?;
		let mut conn2 = CDPConnection::new(ws_url, None).await?;

		let _: GetTargetResponse = conn1.send(
			CDPMessage::root(2, CDPMethod::GetTargets)
		).await?;
		let _: GetTargetResponse = conn2.send(
			CDPMessage::root(2, CDPMethod::GetTargets)
		).await?;

		Ok(())
	}

	#[test]
	fn message_json_representation() -> Result<()>{
		let method = CDPMessage::get_targets(20, "abc");
		let json = method.json()?;

		assert_eq!(json,json!({
			"id": 20,
			"sessionId":"abc",
			"method": "Target.getTargets"
		}));

		Ok(())
	}
}
