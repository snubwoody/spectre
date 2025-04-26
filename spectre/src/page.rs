use crate::{
    cdp::{
        runtime::EvaluateResponse, 
		CDPConnection, 
		CDPMethod, 
		CDPSession, 
		GetDocumentResponse, 
		PageNavigateResponse, 
    }, dom::{DomNode, NodeName}, Result
};

#[derive(Debug)]
pub struct Page {
    session: CDPSession,
    endpoint: String,
}

impl Page {
    pub async fn new(url: &str) -> Result<Self> {
        let conn = CDPConnection::new(url).await?;
		let session = conn.create_session().await?;
		
        Ok(Page {
            session,
            endpoint: String::from(url),
        })
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

	/// Get the whole DOM
    pub async fn get_dom(&mut self) -> Result<DomNode> {
        // Set to -1 to get all sub nodes.
        let method = CDPMethod::GetDocument { depth: -1 };
		let response: GetDocumentResponse = self.session.send(method).await?;
        let root = response.body().root;

        Ok(root)
    }

	pub async fn evaluate(&mut self,expr: &str) -> Result<EvaluateResponse>{
		self.session.evaluate(expr).await
	}

    /// Get an element by it's name.
    pub async fn get_by_tag(&mut self, tag: &str) -> Result<Option<DomNode>> {
        let expr = format!("
			document.getElementsByTagName('{}')
		",tag);

		let result = self.session.evaluate(&expr).await?;
		dbg!(&result);

        Ok(None)
    }

    /// Get an element by it's class name.
	/// 
	/// # Example
	/// 
	/// ```no_run
	/// use spectre::{Page,Result};
	/// 
	/// #[tokio::main]
	/// async fn main() -> Result<()>{
	///     let page = Page::new("").await?;
	///     let button = page.get_by_class("btn-primary").await?;
	///     
	///     Ok(())
	/// }
	/// ```
    pub async fn get_by_class(&mut self, class_name: &str) -> Result<Option<DomNode>> {
        let root = self.get_dom().await?;
		dbg!(root);
        Ok(None)
    }

    /// Get an element by it's name.
    pub async fn get_by_name(&mut self, name: NodeName) -> Result<Option<DomNode>> {
        let root = self.get_dom().await?;
        Ok(root.get_by_name(&name))
    }

    pub async fn navigate(&mut self,url: &str) -> Result<()> {
        let method = CDPMethod::Navigate {
            url: String::from(url),
        };
        let _: PageNavigateResponse = self.session.send(method).await?;

		Ok(())
    }

}