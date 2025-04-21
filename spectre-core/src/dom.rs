use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DomNode {
    pub node_id: i32,
    pub node_value: String,
    pub local_name: String,
    pub document_url: Option<String>,
    pub node_name: NodeName,
    pub parent_id: Option<i32>,
    #[serde(default)]
    pub children: Vec<Box<DomNode>>,
}

impl DomNode {
    /// Returns the first [`DomNode`] that matches the name
    pub fn get_by_name(&self, name: &NodeName) -> Option<Self> {
        if &self.node_name == name {
            return Some(self.clone());
        }

        for child in &self.children {
            if child.get_by_name(name).is_some() {
                return Some(*child.clone());
            }
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum NodeName {
    /// The root document
    #[serde(rename = "#document")]
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
