use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::net::TcpStream;
use std::process::{Child, Command, Stdio};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::{cdp::{AttachToTargetResponse, CDPConnection, CDPMessage, CDPMethod, CreateTargetResponse, Target, TargetResponse}, error::CDPError, Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::page::Page;

pub struct Browser {
    process: Child,
	conn: CDPConnection,
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
		let conn = CDPConnection::root(&ws_url).await?;

        Ok(
			Self {
				process: child,
				conn, 
				url:ws_url,
			}
		)
    }

	pub fn url(&self) -> &str{
		&self.url
	}

	pub async fn get_targets(&mut self) -> Result<Vec<Target>>{

		let response: TargetResponse = self.conn.send(
			CDPMessage::root(1, CDPMethod::GetTargets)
		).await?;
		
		Ok(response.targets())
	}

	pub async fn goto(&mut self, url: &str) -> Result<Page>{
		let method = CDPMethod::CreateTarget { url: String::from(url) };
		let message = CDPMessage::root(1, method);
		let response: CreateTargetResponse = self.conn.send(message).await?;

		let method = CDPMethod::AttachToTarget { target_id: response.target_id().to_string() };
		let message =  CDPMessage::root(1, method); 

		let response: AttachToTargetResponse = self.conn.send(message).await?;
		let page = Page::new(response.session_id(),&self.url).await?;

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
	async fn goto_page() -> Result<()>{
		let mut browser = Browser::launch().await?;
		let page = browser.goto("https://youtube.com");
		
		Ok(())
	}
}
