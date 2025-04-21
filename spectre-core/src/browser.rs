use crate::Error;
use crate::cdp::WebSocketTarget;
use crate::page::Page;
use crate::{
    Result,
    cdp::{
        AttachToTargetResponse, CDPConnection, CDPMessage, CDPMethod, CreateTargetResponse,
        GetTargetResponse, Target,
    },
};
use reqwest::{Client, Method, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::{Child, Command, Stdio};

/// An instance of a browser
///
/// # Example
/// ```no_run
/// use spectre::{Browser,Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()>{
/// 	let browser = Browser::launch().await?;
/// 	Ok(())
/// }
/// ```
///
/// The browser is automatically closed when dropped.
///
/// ```ignore
/// use spectre::Browser;
///
/// impl Drop for Browser{
/// 	fn drop(&mut self){
/// 		self.proccess
/// 			.kill()
/// 			.expect("Failed to close browser");
/// 	}
/// }
/// ```
pub struct Browser {
    process: Child,
    conn: CDPConnection,
    /// The local network address of chrome
    url: String,
    message_id: i32,
    port: u16,
}

impl Browser {
    /// Launch a new browser
    pub async fn launch() -> Result<Self> {
        // Get any available port
        let listener = std::net::TcpListener::bind("0.0.0.0:0")?;
        let port = listener.local_addr()?.port();

        std::mem::drop(listener);

        let child = Command::new("../chrome-win64/chrome.exe")
            .args([
                "--headless",
                "--disable-gpu",
                "--no-sandbox",
                &format!("--remote-debugging-port={}", port),
            ])
            .stdout(Stdio::null()) // Silence output
            .spawn()?;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ResponseBody {
            web_socket_debugger_url: String,
        }

        let response = reqwest::get(format!("http://localhost:{}/json/version", port)).await?;
        let body: ResponseBody = response.json().await?;

        let ws_url = body.web_socket_debugger_url;
        let conn = CDPConnection::root(&ws_url).await?;

        Ok(Self {
            process: child,
            conn,
            url: ws_url,
            message_id: 0,
            port,
        })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub async fn get_targets(&mut self) -> Result<Vec<Target>> {
        let response: GetTargetResponse = self
            .conn
            .send(CDPMessage::root(self.message_id, CDPMethod::GetTargets))
            .await?;

        self.message_id += 1;
        Ok(response.body().targets)
    }

    pub async fn goto(&mut self, url: &str) -> Result<Page> {
        let client = Client::new();
        let resp = client
            .put(format!("http://localhost:{}/json/new?{}", self.port, url))
            .send()
            .await?;

        let body: WebSocketTarget = resp.json().await?;
        let page = Page::new("", &body.endpoint).await?;
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn goto_page() -> Result<()> {
        let mut browser = Browser::launch().await?;
        let _ = browser.goto("https://youtube.com").await?;
        let targets = browser.get_targets().await?;
        targets
            .iter()
            .find(|t| t.url() == "https://www.youtube.com/")
            .unwrap();

        Ok(())
    }
}
