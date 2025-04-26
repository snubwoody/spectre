//! Browser automation and testing library.
mod browser;
pub mod cdp;
pub mod dom;
mod error;
mod page;
pub use browser::Browser;
pub use error::{Error, Result};
pub use page::Page;
pub use spectre_macros::test;

pub const EMPTY_PAGE: &str = "https://blank.org/";
