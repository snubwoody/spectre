use serde::{Deserialize,Serialize};


#[derive(Debug,Serialize,Deserialize,Clone,PartialEq, Eq)]
#[serde(rename_all="camelCase")]
pub struct DomNode{
	pub node_id: i32,
	pub node_value: String,
	pub local_name: String,
	pub document_url: Option<String>,
	pub node_name: NodeName,
	pub parent_id: Option<i32>,
	#[serde(default)]
	pub children: Vec<Box<DomNode>>
}

#[derive(Debug,Serialize,Deserialize,Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all="UPPERCASE")]
pub enum NodeName {
	/// The root document
	#[serde(rename="#document")]
	Document,
	/// Inner text
	#[serde(rename="#text")]
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
	#[serde(alias="html")]
	Html,
	Body, 
	Head
}