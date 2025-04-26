use serde::{Deserialize, Serialize};
use crate::cdp::CDPSession;

/// An element in the DOM, all elements are matched not just
/// html tags including `#document` and `#text`.
#[derive(Debug)]
pub struct Element{
	node_id: i32,
	session: CDPSession,
	children: Vec<Box<Element>>
}

impl Element{
	fn new(node_id: i32,session: CDPSession, children: Vec<Box<Element>>) -> Self{
		Self { node_id, session,children }
	}
}


/// Html attributes
#[derive(Debug,Clone, PartialEq, Eq)]
pub enum Attribute{
	Href(String),
	Id(String),
	Class(String),
	Unknown(String)
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq,Default)]
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

impl DomNode {
    /// Returns the first [`DomNode`] that matches the name
    pub fn get_by_name(&self, name: &NodeName) -> Option<Self> {
        if &self.node_name == name {
			return Some(self.clone());
        }
		
        for child in &self.children {
			if &child.node_name == name{
				return Some(*child.clone());
			}

			child.get_by_name(name);
        }

        None
    }

	pub fn into_element(&self,session: CDPSession) -> Element{
		let mut children = vec![];
		
		for child in &self.children{
			let child_element = child.into_element(session.clone());
			children.push(Box::new(child_element));
		}

		Element::new(self.node_id,session, children)
	}
}

#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord,Default)]
#[derive(Serialize,Deserialize)]
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

#[cfg(test)]
mod tests{
    use super::*;

	#[test]
	fn get_dom_node(){
		let div_id:i32 = rand::random();
		let link_id:i32 = rand::random();
		
		let link = DomNode{
			node_id: link_id,
			node_name: NodeName::A,
			..Default::default()
		};

		let div = DomNode{
			node_id: div_id,
			node_name: NodeName::Div,
			children: vec![Box::new(link)],
			..Default::default()
		};

		let root = DomNode{
			children: vec![Box::new(div)],
			..Default::default()
		};

		let div = root.get_by_name(&NodeName::Div);
		assert_eq!(div.unwrap().node_id,div_id);

		let img = root.get_by_name(&NodeName::Img);
		assert!(img.is_none());
	}
}