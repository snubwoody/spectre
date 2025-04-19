use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::net::TcpStream;
use std::process::{Child, Command, Stdio};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::{Error,Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug,Deserialize,Serialize)]
pub enum CDPMessage{
	GetTargets(i32)
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
	/// 	"method": "Targets.getTargets"
	/// }));
	/// ```
	pub fn json(&self) -> Value{
		match self{
			Self::GetTargets(id) => json!({"id":id,"method":"Targets.getTargets"})
		}
	}
}

#[derive(Debug,Serialize,Deserialize)]
struct TargetResponse{
	id: i32,
	result: TargetResult
}


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
struct TargetResult{
	target_infos: Vec<Target>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
struct Target{
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

#[derive(Debug,Serialize,Deserialize)]
struct CreateTargetResponse{
	id: i32,
	result: TargetResult
}


pub struct Browser {
    process: Child,
	stream: WebSocketStream<MaybeTlsStream<TcpStream>>
}

impl Browser {
    pub async fn launch() -> Result<Self> {
		let port = match std::env::var("SPECTRE_PORT"){
			Ok(var) => var,
			Err(_)=> String::from("5000")
		};

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
		let (stream, _) = connect_async(ws_url).await?;
		println!("Connected to chrome websocket");

        Ok(
			Self {
				process: child,
				stream 
			}
		)
    }

	/// Send a message to the browser using the cdp protocol
	pub async fn send<T>(&mut self) -> Result<T>
	where T: DeserializeOwned
	{
		let message = Message::text(json!({
			"id": 1,
			"method":"Target.createTarget",
			"params":{
				"url":"https://youtube.com"
			}
		}).to_string());

		self.stream.send(message).await?;

		if let Some(Ok(Message::Text(message))) = self.stream.next().await{
			let response: T = serde_json::from_str(&message.to_string())?;
			return Ok(response);
		}

		Err(Error::FailedToSendMessage)
	}
}

impl Drop for Browser {
    fn drop(&mut self) {
		// FIXME not killing all processes
        // Kill the process with the broswer
        self.process
            .kill()
            .expect("Process should have been killed");
    }
}

#[cfg(test)]
mod tests{
	use futures_util::{SinkExt, StreamExt};
	use serde_json::{json, Value};
	use tokio_tungstenite::tungstenite::Message;
	
	use super::*;

	#[tokio::test]
	async fn start_browser() -> Result<()>{
		let browser = Browser::launch().await?;
		let response = reqwest::get(format!("http://localhost:{}/json/version", 5000)).await?;
		Ok(())
	}

	#[tokio::test]
	async fn startup_browser() -> Result<()>{
		let mut browser = Browser::launch().await?;
		let message = Message::text(json!({
			"id": 1,
			"method":"Target.createTarget",
			"params":{
				"url":"https://youtube.com"
			}
		}).to_string());

		browser.stream.send(message).await?;
		if let Some(Ok(Message::Text(message))) = browser.stream.next().await{
			let response: Value = serde_json::from_str(&message.to_string())?;
			dbg!(response);
		}

		let message = Message::text(json!({
			"id": 1,
			"method":"Target.getTargets"
		}).to_string());

		browser.stream.send(message).await?;
		if let Some(Ok(Message::Text(message))) = browser.stream.next().await{
			let response: TargetResponse = serde_json::from_str(&message.to_string())?;
			dbg!(response);
		}

		Ok(())
	}
}
