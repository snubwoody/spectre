//! Browser automation and testing library. Communication is done via
//! the [chrome devtools protocol](https://chromedevtools.github.io/devtools-protocol/)
//! which sends json messages back and forth. Currently only chrome is supported, but
//! firefox and safari support is planned.
//!
//! # Getting started
//! ```
//! use spectre::{Browser,Page,Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()>{
//!     let mut browser = Browser::start().await?;
//!     let page = browser.goto("https://www.example.com").await?;
//!     let url = page.url().await?;
//!     
//!     assert_eq!(&url,"https://www.example.com/");
//!     Ok(())
//! }
//! ```
//!
//! # Pages
//!
//! [`Page`]'s are individual chrome tabs that can be controlled.
pub mod browser;
pub mod cdp;
pub mod dom;
pub mod download;
mod error;
mod page;
mod web;
pub use browser::Browser;
pub use error::{Error, Result};
pub use page::Page;
pub use spectre_macros::test;

pub const EMPTY_PAGE: &str = "https://blank.org/";

/// Get any available port on the device
pub async fn get_available_port() -> Result<u16> {
    // Get any available port
    let listener = std::net::TcpListener::bind("[::1]:0")?;
    let port = listener.local_addr()?.port();

    // Immediately drop the listener to free the port
    std::mem::drop(listener);

    Ok(port)
}
