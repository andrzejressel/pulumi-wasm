use rmpv::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MaybeNodeValue {
    NotYetCalculated,
    Set(NodeValue),
}

impl From<Value> for MaybeNodeValue {
    fn from(value: Value) -> Self {
        Self::Set(value.into())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NodeValue {
    Nothing, // preview
    Exists(Value),
}

impl From<Value> for NodeValue {
    fn from(value: Value) -> Self {
        Self::Exists(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionName(String);

impl From<FunctionName> for String {
    fn from(val: FunctionName) -> Self {
        val.0
    }
}

impl From<String> for FunctionName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FunctionName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutputId(Uuid);

impl From<String> for OutputId {
    fn from(value: String) -> Self {
        Self(Uuid::parse_str(&value).unwrap())
    }
}

impl fmt::Display for OutputId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for OutputId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldName(String);

impl FieldName {
    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl From<String> for FieldName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FieldName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<&String> for FieldName {
    fn from(value: &String) -> Self {
        Self(value.to_string())
    }
}
