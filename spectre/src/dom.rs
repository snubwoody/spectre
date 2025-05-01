use crate::{Result, cdp::CdpSession};
use serde::{Deserialize, Serialize};

/// An element in the DOM, all elements are matched not just
/// html tags including `#document` and `#text`.
#[derive(Debug)]
pub struct Element {
    node_id: i32,
    session: CdpSession,
}

impl Element {
    pub fn new(node_id: i32, session: CdpSession) -> Self {
        Self { node_id, session }
    }

    async fn resolve_node(&self) -> Result<()> {
        let node = self.session.resolve_node(self.node_id).await?;
        dbg!(node);

        Ok(())
    }

    pub async fn get_attributes(&self) -> Result<()> {
        self.resolve_node().await?;
        Ok(())
    }
}

/// Html attributes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    Href(String),
    Id(String),
    Class(String),
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DomNode {
    pub node_id: i32,
    pub node_value: String,
    pub local_name: String,
    pub document_url: Option<String>,
    pub node_name: NodeName,
    pub parent_id: Option<i32>,
    #[serde(default)]
    pub attributes: Vec<String>,
    #[serde(default)]
    pub children: Vec<Box<DomNode>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NodeName {
    /// The root document
    #[serde(rename = "#document")]
    #[default]
    Document,
    /// Inner text
    #[serde(rename = "#text")]
    Text,
    Title,
    A,
    Meta,
    P,
    Style,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Br,
    Img,
    Div,
    // FIXME this is breaks serialization, it's the only
    // node that is sent in lowercase
    #[serde(alias = "html")]
    Html,
    Body,
    Head,
    Script,
    Link,
    // FIXME also broken
    #[serde(alias = "svg")]
    Svg,
    Abbr,
    Area,
    Article,
    Aside,
    Audio,
    B,
    Base,
    Code,
    Col,
    Data,
    DataList,
    Form,
    Input,
    Label,
    Button,
    Iframe,
    Li,
    Ul,
    Ol,
    Main,
    Menu,
    Header,
    #[serde(alias = "path")]
    Path,
    NoScript,
    #[serde(alias = "image")]
    Image,
    Nav,
    Select,
    Search,
    Section,
    Span,
    Table,
    Dialog,
    TextArea,
    Hr,
    #[serde(alias = "rect")]
    Rect,
    Track,
    Video,
    Tr,
    Center,
    #[serde(alias = "g")]
    G,
    #[serde(alias = "circle")]
    Circle,
    // Any unknown or custom elements
    #[serde(untagged)]
    Unknown(String),
}
