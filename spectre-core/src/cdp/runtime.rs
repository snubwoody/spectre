use super::CDPResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

pub type EvaluateResponse = CDPResponse<EvaluateBody>;

#[derive(Debug, Deserialize,Clone)]
pub struct EvaluateBody {
    pub result: RemoteObject,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    pub column_number: i32,
    pub line_number: i32,
    pub exception_id: i32,
    pub text: String,
    pub script_id: Option<String>,
}

/// Mirror object referencing the original javascript object
///
/// Corresponds to [`Runtime.RemoteObject`](https://vanilla.aslushnikov.com/?Runtime.RemoteObject)
#[derive(Debug, Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject {
    #[serde(rename = "type")]
    pub object_type: RemoteObjectType,
    /// String representation of the object
    pub description: String,
    /// The object class name (for object types only)
    pub class_name: Option<String>,
    // TODO this has the 'any' type so replace with valid options
    pub value: Option<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RemoteObjectType {
    Object,
    Function,
    Undefined,
    String,
    Number,
    Boolean,
    Symbol,
    Bigint,
}
