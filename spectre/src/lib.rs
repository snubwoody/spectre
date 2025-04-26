//! Browser automation and testing library.
mod browser;
mod page;
mod error;
pub mod dom;
pub mod cdp;
pub use browser::Browser;
pub use error::{Error, Result};
pub use page::Page;
pub use spectre_macros::test;

pub const EMPTY_PAGE: &str = "https://blank.org/";
