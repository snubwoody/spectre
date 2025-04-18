mod error;
pub use error::Error;
use futures_util::{StreamExt, future, pin_mut};
use std::{
    io::{Cursor, Read},
    process::{Child, Command},
};
use tokio_tungstenite::{connect_async, tungstenite::http::response};

async fn download_chrome() -> Result<(), Error> {
    println!("Downloading chrome...");
    let url = "https://storage.googleapis.com/chrome-for-testing-public/135.0.7049.95/win64/chrome-win64.zip";
    let response = reqwest::get(url).await?;

    let bytes = response.bytes().await?;
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;

    println!("Unzipping...");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        // FIXME
        let outpath = file.enclosed_name().unwrap();

        if file.is_dir() {
            std::fs::create_dir_all(outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut out_file = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }

    Ok(())
}

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
                &format!("--remote-debugging-port={}", 5000),
            ])
            .spawn()?;

        let response = reqwest::get(format!("http://localhost:{}/json/version", port)).await?;
        let ws_url = response.text().await?;
        dbg!(&ws_url);
        let (ws, _) = connect_async(ws_url).await?;
        println!("Connected to chrome websocket");
        let (write, read) = ws.split();
        dbg!(&write);
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let browser = Browser::launch().await?;
    Ok(())
}
