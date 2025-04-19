use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::net::TcpStream;
use std::process::{Child, Command, Stdio};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::{error::CDPError, Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug,Deserialize,Serialize)]
pub enum CDPMessage{
	GetTargets(i32),
	CreateTarget{
		id: i32,
		url: String
	},
	AttachToTarget{
		id: i32,
		target_id: String
	}
}

impl CDPMessage{
	/// Get a JSON representation of the message
	/// # Example
	/// ```
	/// use spectre::browser::CDPMessage;
	/// use serde_json::json;
	/// 
	/// let message = CDPMessage::GetTargets(200);
	/// let json = message.json();
	/// 
	/// assert_eq!(json,json!({
	/// 	"id": 200,
	/// 	"method": "Target.getTargets"
	/// }));
	/// ```
	pub fn json(&self) -> Value{
		match self{
			Self::GetTargets(id) => json!({"id":id,"method":"Target.getTargets"}),
			Self::CreateTarget { id, url } => json!({
				"id": id,
				"method": "Target.createTarget",
				"params": {
					"url": url
				}
			}),
			Self::AttachToTarget { id, target_id } => json!({
				"id":id,
				"method": "Target.attachToTarget",
				"params":{
					"targetId": target_id,
					"flatten": true
				}
			})
		}
	}
}

impl Into<Message> for CDPMessage{
	fn into(self) -> Message {
		Message::text(self.json().to_string())
	}
}

#[derive(Debug,Serialize,Deserialize)]
struct TargetResponse{
	id: i32,
	result: TargetResult
}

#[derive(Debug,Serialize,Deserialize)]
struct AttachToTargetResponse{
	method: String,
	params: AttachToTargetBody
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
struct AttachToTargetBody{
	session_id: String,
	target_info: Target,
	waiting_for_debugger:bool
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
struct CreateTargetResponse{
	id: i32,
	result: CreateTargetBody
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
struct CreateTargetBody{
	target_id: String
}


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
struct TargetResult{
	target_infos: Vec<Target>
}

#[derive(Debug,Serialize,Deserialize)]
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

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="snake_case")]
enum TargetType{
	Tab,
	Page,
	Iframe,
	Worker,
	ServiceWorker,
	Browser,
	WebView
}

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

	/// Send a message 
	pub async fn send<T>(&mut self,message: CDPMessage) -> Result<T>
	where T: DeserializeOwned
	{
		let msg: Message = message.into();
		self.stream.send(msg).await?;

		if let Some(Ok(Message::Text(message))) = self.stream.next().await{
			match serde_json::from_str(&message.to_string()) {
				Ok(response) => {
					return Ok(response);
				}
				Err(_)=>{
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
}
