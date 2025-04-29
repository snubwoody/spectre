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
mod browser;
pub mod download;
pub mod cdp;
pub mod dom;
mod error;
mod page;
mod web;
pub use browser::Browser;
pub use error::{Error, Result};
pub use page::Page;
pub use spectre_macros::test;

pub const EMPTY_PAGE: &str = "https://blank.org/";
