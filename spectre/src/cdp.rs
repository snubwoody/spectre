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
	#[serde(rename="Target.createTargets")]
	CreateTarget{
		url: String
	},
	#[serde(rename="Target.attachToTarget")]
	AttachToTarget{
		target_id: String
	}
}

// impl Into<Message> for CDPMessage{
// 	fn into(self) -> Message {
// 		Message::text(self.json().to_string())
// 	}
// }

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
		let msg: Message = Message::Text(message.json()?.to_string().into());

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
	use super::*;

	#[test]
	fn send_cdp_message(){

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
