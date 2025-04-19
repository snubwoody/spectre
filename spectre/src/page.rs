use crate::{cdp::CDPConnection, Result};

pub struct Page{
	session_id: String,
	conn: CDPConnection
}

impl Page{
	pub async fn new(session_id: &str,url:&str) -> Result<Self>{
		let conn = CDPConnection::new(url,Some(session_id.to_string())).await?;
		Ok(Page { 
			session_id: String::from(session_id),
			conn 
		})
	}
}