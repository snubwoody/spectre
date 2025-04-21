pub mod browser;
pub mod cdp;
pub mod dom;
mod error;
pub mod page;
pub use browser::Browser;
pub use error::{Error, Result};
pub use page::Page;

pub const EMPTY_PAGE: &'static str = "https://www.webpagetest.org/blank.html";