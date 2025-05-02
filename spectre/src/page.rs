use crate::{
    Error, Result,
    cdp::{
        CdpConnection, CdpMethod, CdpSession, GetDocumentResponse, PageNavigateResponse,
        runtime::EvaluateResponse,
    },
    dom::{DomNode, Element},
};

#[derive(Debug)]
pub struct Page {
    session: CdpSession,
    endpoint: String,
}

impl Page {
    pub async fn new(url: &str) -> Result<Self> {
        let conn = CdpConnection::new(url).await?;
        let session = conn.create_session().await?;

        Ok(Page {
            session,
            endpoint: String::from(url),
        })
    }

    /// Get the url of the page.
    ///
    /// # Example
    ///
    /// ```
    /// use spectre::{Browser,Page,Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()>{
    ///     let mut browser = Browser::start().await?;
    ///     let page = browser.goto("https://www.youtube.com").await?;
    ///     
    ///     let url = page.url().await?;
    ///     assert_eq!(&url,"https://www.youtube.com/");
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn url(&self) -> Result<String> {
        let response = self.session.evaluate("document.URL").await?;
        let body = response.body();
        let value = body
            .result
            .value
            .ok_or(Error::PageError(String::from("Failed to get page url")))?;
        // FIXME make above error more descriptive

        // FIXME do not unwrap
        let url = value.as_str().unwrap();

        Ok(url.to_owned())
    }

    /// Locate the first element whose class name matches
    /// the input class.
    ///
    /// # Example
    /// ```
    /// use spectre::{Result,Page,Browser};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()>{
    ///     let mut browser = Browser::start().await?;
    ///     let page = browser.new_page().await?;
    ///
    ///     let element = page.locate_by_class("does-not-exist").await?;
    ///     assert!(element.is_none());
    ///     Ok(())
    /// }
    /// ```
    pub async fn locate_by_class(&self, class: &str) -> Result<Option<Element>> {
        let root = self.get_dom().await?;
        let element = self
            .session
            .query_selector(root.node_id, &format!(".{class}"))
            .await?;

        Ok(element)
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get the whole DOM
    pub async fn get_dom(&self) -> Result<DomNode> {
        // Set to -1 to get all sub nodes.
        let method = CdpMethod::GetDocument { depth: -1 };
        let response: GetDocumentResponse = self.session.send(method).await?;
        let root = response.body().root;

        Ok(root)
    }

    pub async fn evaluate(&mut self, expr: &str) -> Result<EvaluateResponse> {
        self.session.evaluate(expr).await
    }

    /// Close the page
    pub async fn close(&self) -> Result<()> {
        self.session.close_page().await
    }

    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        let method = CdpMethod::Navigate {
            url: String::from(url),
        };
        let _: PageNavigateResponse = self.session.send(method).await?;

        Ok(())
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        // I don't really know better (simple) way to do this
        // it should be okay in most scenarios
        let session = self.session.clone();
        tokio::spawn(async move{
            session.close_page().await
        });
    }
}