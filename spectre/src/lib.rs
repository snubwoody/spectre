mod error;
pub use error::Error;
use futures_util::{StreamExt, future, pin_mut};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    io::{Cursor, Read},
    process::{Child, Command},
};
use tokio_tungstenite::{connect_async, tungstenite::http::response};

pub struct Browser {
    process: Child,
}

impl Browser {
    pub async fn launch() -> Result<Self, Error> {
        let port = 5000;
        let mut child = Command::new("chrome-win64/chrome.exe")
            .args(&[
                "--headless",
                "--disable-gpu",
                "--no-sandbox",
                &format!("--remote-debugging-port={}", 5000),
            ])
            .spawn()?;

		#[derive(Debug,Serialize,Deserialize)]
		#[serde(rename_all="camelCase")]
		struct ResponseBody{
			web_socket_debugger_url: String,
			
		}

        let response = reqwest::get(format!("http://localhost:{}/json/version", port)).await?;
        let body: ResponseBody = response.json().await?;

		dbg!(&body);
		let ws_url = body.web_socket_debugger_url;
		let (ws, _) = connect_async(ws_url).await?;
		let (write, read) = ws.split();
		dbg!(&read);
		dbg!(&write);
		println!("Connected to chrome websocket");

        // dbg!(&write);
        Ok(Self { process: child })
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        // Kill the process with the broswer
        self.process
            .kill()
            .expect("Process should have been killed");
    }
}
