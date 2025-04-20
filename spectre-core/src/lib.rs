//! Browser automation and testing library.
pub mod browser;
pub mod cdp;
mod error;
pub mod page;
pub use error::{Error, Result};
pub use browser::Browser;
pub use page::Page;
