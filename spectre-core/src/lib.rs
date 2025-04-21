pub mod browser;
pub mod cdp;
pub mod dom;
mod error;
pub mod page;
pub use error::{Error, Result};
pub use browser::Browser;
pub use page::Page;
