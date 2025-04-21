use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use super::CDPResponse;

pub type EvaluateResponse = CDPResponse<EvaluateBody>;

#[derive(Debug, Deserialize)]
pub struct EvaluateBody{
	result: RemoteObject,
	#[serde(rename="exeptionDetails")]
	exception_details: Option<ExceptionDetails>
}

#[derive(Debug,Deserialize,Clone,PartialEq,Eq)]
#[serde(rename_all="camelCase")]
pub struct ExceptionDetails{
	pub column_number: i32,
	pub line_number: i32,
	pub exception_id: i32,
	pub text: String,
	pub script_id: Option<String>
}

/// Mirror object referencing the original javascript object
/// 
/// Corresponds to [`Runtime.RemoteObject`](https://vanilla.aslushnikov.com/?Runtime.RemoteObject)
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RemoteObject{
	#[serde(rename="type")]
	object_type: RemoteObjectType,
	/// String representation of the object
	description: String,
	/// The object class name (for object types only)
	class_name: String,
	// TODO this has the 'any' type so replace with valid options
	value: Value
}

#[derive(Debug,Clone, Copy,PartialEq, Eq, PartialOrd, Ord,Serialize,Deserialize)]
#[serde(rename_all="lowercase")]
pub enum RemoteObjectType{
	Object,
	Function,
	Undefined,
	String,
	Number,
	Boolean,
	Symbol,
	Bigint
}
