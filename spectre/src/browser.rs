use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::net::TcpStream;
use std::process::{Child, Command, Stdio};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::{cdp::CDPConnection, error::CDPError, Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::page::Page;

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

pub struct Browser {
    process: Child,
	stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
	/// The local network address of chrome 
	url: String
}

impl Browser {
    pub async fn launch() -> Result<Self> {
		
		// Get any available port
		let listener = std::net::TcpListener::bind("0.0.0.0:0")?;
		let port = listener.local_addr()?.port();

		std::mem::drop(listener);

        let child = Command::new("../chrome-win64/chrome.exe")
            .args(&[
                "--headless",
                "--disable-gpu", 
                "--no-sandbox",
                &format!("--remote-debugging-port={}", port),
            ])
			.stdout(Stdio::null()) // Silence output
            .spawn()?;

		#[derive(Debug,Serialize,Deserialize)]
		#[serde(rename_all="camelCase")]
		struct ResponseBody{
			web_socket_debugger_url: String,
			
		}

        let response = reqwest::get(format!("http://localhost:{}/json/version", port)).await?;
        let body: ResponseBody = response.json().await?;

		let ws_url = body.web_socket_debugger_url;
		let (stream, _) = connect_async(&ws_url).await?;
		println!("Connected to chrome websocket");

        Ok(
			Self {
				process: child,
				stream, 
				url:ws_url,
			}
		)
    }

	pub async fn url(&self) -> &str{
		&self.url
	}

	pub async fn get_targets(&mut self) -> Result<Vec<Target>>{
		let response: TargetResponse = self.send(CDPMessage::GetTargets(0)).await?;
		
		Ok(response.result.target_infos)
	}

	/// Send a message to the browser using the cdp protocol
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

	pub async fn goto(&mut self, url: &str) -> Result<Page>{
		let message = CDPMessage::CreateTarget { id: 1, url: String::from(url) };
		let response: CreateTargetResponse = self.send(message).await?;

		let message = CDPMessage::AttachToTarget { id: 2, target_id:response.result.target_id }; 
		let response: AttachToTargetResponse = self.send(message).await?;
		let page = Page::new(&response.params.session_id,&self.url).await?;

		Ok(page)
	}
}

impl Drop for Browser {
    fn drop(&mut self) {
		// TODO close the browser gracefully first
		// https://vanilla.aslushnikov.com/?Browser.close
		
        // Don't leave zombie processes
        self.process
            .kill()
            .expect("Process should have been killed");
    }
}


#[cfg(test)]
mod tests{
	use serde_json::Value;
	use super::*;

	#[tokio::test]
	async fn send_cdp_message() -> Result<()>{
		let mut browser = Browser::launch().await?;
		let message = CDPMessage::CreateTarget { id: 1, url: String::from("https://youtube.com") };
		browser.send::<Value>(message).await?;

		let response: TargetResponse = browser.send(CDPMessage::GetTargets(2)).await?;

		for target in response.result.target_infos.iter(){
			if target.url == "https://www.youtube.com/"{
				return Ok(());
			}
		}
		
		panic!();
	}

	#[tokio::test]
	async fn goto_page() -> Result<()>{
		let mut browser = Browser::launch().await?;
		let page = browser.goto("https://youtube.com");
		
		Ok(())
	}
}
