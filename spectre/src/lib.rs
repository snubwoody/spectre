//! Browser automation and testing library.
pub use spectre_macros::test;
pub mod browser;
pub mod dom;
mod error;
pub mod cdp;
pub mod page;
pub use browser::Browser;
pub use error::{Error, Result};
pub use page::Page;

pub const EMPTY_PAGE: &str = "https://blank.org/";
