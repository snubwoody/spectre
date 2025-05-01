use crate::cdp::{CdpSession, WebSocketTarget};
use crate::page::Page;
use crate::{Error, get_available_port};
use crate::{
    Result,
    cdp::{CdpConnection, CDPMessage, CdpMethod, GetTargetResponse, Target},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// An instance of a browser. The browser is started on a
/// local port and listens to json messages via websockets.
///
/// # Start a new browser process
///
/// ```no_run
/// use spectre::{Browser,Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()>{
///     let browser = Browser::start().await?;
///     Ok(())
/// }
/// ```
///
/// Browsers are automatically cleaned up when the value is dropped.
///
/// ```ignore
/// use spectre::Browser;
///
/// impl Drop for Browser{
///     fn drop(&mut self){
///         self.proccess
///             .kill()
///             .expect("Failed to close browser");
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Browser {
    /// The process the browser is running on. This is
    /// `None` if we connected to an already running
    /// browser (`Browser::connect`) instead of starting one.
    process: Option<Child>,
    conn: CdpConnection,
    /// The local network address of chrome
    url: String,
    message_id: i32,
    kill_on_drop: bool,
    port: u16,
}

impl Browser {
    pub async fn is_running(port: u16) -> bool {
        let response = reqwest::get(format!("http://localhost:{}/json/version", port)).await;
        if response.is_ok() {
            return true;
        }

        false
    }

    pub fn kill_on_drop(&mut self, value: bool) {
        self.kill_on_drop = value;
    }

    /// Start the browser child process.
    fn start_process(port: u16) -> crate::Result<Child> {
        let home_path = home::home_dir().ok_or(Error::FailedToGetHomeDir)?;
        let spectre_path = home_path.join(".spectre").join("browsers");
        let chrome_path = if cfg!(target_os = "windows") {
            spectre_path.join("chrome-win64/chrome.exe")
        } else if cfg!(target_os = "macos") {
            spectre_path.join("chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing")
        } else {
            spectre_path.join("chrome-linux64/chrome")
        };

        let child = Command::new(chrome_path)
            .args([
                "--headless",
                "--disable-gpu",
                "--no-sandbox",
                &format!("--remote-debugging-port={}", port),
            ])
            .stdout(Stdio::null()) // Silence output
            .spawn()?;

        Ok(child)
    }

    async fn connect_to_process(port: u16) -> crate::Result<(String, CdpConnection)> {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ResponseBody {
            web_socket_debugger_url: String,
        }

        // Wait for the browser to be active
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        let mut elapsed = Duration::default();
        loop {
            interval.tick().await;
            match reqwest::get(format!("http://localhost:{}/json/version", port)).await {
                Ok(response) => {
                    let body: ResponseBody = response.json().await?;
                    let ws_url = body.web_socket_debugger_url;
                    let conn = CdpConnection::new(&ws_url).await?;

                    return Ok((ws_url, conn));
                }
                Err(err) => {
                    if elapsed > Duration::from_secs(3) {
                        return Err(err.into());
                    }
                }
            }
            elapsed += interval.period();
        }
    }
    /// Start a new browser
    ///
    /// # Example
    /// ```
    /// use spectre::Browser;
    ///
    /// #[tokio::main]
    /// async fn main() -> spectre::Result<()>{
    ///     let browser = Browser::start().await?;
    /// }
    /// ```
    pub async fn start() -> Result<Self> {
        let port = get_available_port().await?;
        let child = Self::start_process(port)?;
        let (url, conn) = Self::connect_to_process(port).await?;

        Ok(Self {
            process: Some(child),
            conn,
            kill_on_drop: true,
            url,
            message_id: 0,
            port,
        })
    }

    /// Start a new browser on a specific port
    pub async fn start_on(port: u16) -> Result<Self> {
        let child = Self::start_process(port)?;
        let (url, conn) = Self::connect_to_process(port).await?;

        Ok(Self {
            process: Some(child),
            conn,
            url,
            kill_on_drop: true,
            message_id: 0,
            port,
        })
    }

    /// Connect to a running browser instance
    pub async fn connect(port: u16) -> Result<Self> {
        let (url, conn) = Self::connect_to_process(port).await?;

        Ok(Self {
            process: None,
            conn,
            url,
            kill_on_drop: true,
            message_id: 0,
            port,
        })
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub async fn get_targets(&mut self) -> Result<Vec<Target>> {
        let response: GetTargetResponse = self
            .conn
            .send(CDPMessage::root(self.message_id, CdpMethod::GetTargets))
            .await?;

        self.message_id += 1;
        Ok(response.body().targets)
    }

    /// Get the session for the default browser page.
    ///
    /// # Example
    /// ```
    /// use spectre::{Browser,Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()>{
    ///     let mut browser = Browser::start().await?;
    ///     let session = browser.get_session().await;
    ///     
    ///     assert!(session.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_session(&mut self) -> Result<CdpSession> {
        let connection = CdpConnection::new(&self.url).await?;
        let session = connection.create_session().await?;

        Ok(session)
    }

    pub async fn new_page(&self) -> Result<Page> {
        let client = Client::new();
        let resp = client
            .put(format!(
                "http://localhost:{}/json/new?https://example.com",
                self.port
            ))
            .send()
            .await?;

        let body: WebSocketTarget = resp.json().await?;
        let page = Page::new(&body.endpoint).await?;
        Ok(page)
    }

    pub async fn goto(&mut self, url: &str) -> Result<Page> {
        // FIXME going to '.html' pages breaks this
        let client = Client::new();
        let resp = client
            .put(format!("http://localhost:{}/json/new?{}", self.port, url))
            .send()
            .await?;

        let body: WebSocketTarget = resp.json().await?;
        let page = Page::new(&body.endpoint).await?;
        Ok(page)
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        // TODO close the browser gracefully first
        // https://vanilla.aslushnikov.com/?Browser.close
        // Don't leave zombie processes
        if !self.kill_on_drop {
            return;
        }

        if let Some(child) = &mut self.process {
            child.kill().expect("Failed to kill child");
        }
    }
}
