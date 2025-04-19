use crate::page::Page;
use crate::{
    Error, Result,
    cdp::{
        AttachToTargetResponse, CDPConnection, CDPMessage, CDPMethod, CreateTargetResponse,
        GetTargetResponse, Target,
    },
    error::CDPError,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::process::{Child, Command, Stdio};

pub struct Browser {
    process: Child,
    conn: CDPConnection,
    /// The local network address of chrome
    url: String,
    message_id: i32,
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
        let method = CDPMethod::CreateTarget {
            url: String::from(url),
        };
        let message = CDPMessage::root(self.message_id, method);
        let response: CreateTargetResponse = self.conn.send(message).await?;
        self.message_id += 1;

        let method = CDPMethod::AttachToTarget {
            target_id: response.body().target_id,
            flatten: true,
        };
        let message = CDPMessage::root(self.message_id, method);
        let response: AttachToTargetResponse = self.conn.send(message).await?;
        self.message_id += 1;

        let page = Page::new(response.session_id(), &self.url).await?;

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
