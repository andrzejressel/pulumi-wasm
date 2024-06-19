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
pub(crate) struct FunctionName(String);

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
pub(crate) struct OutputId(Uuid);

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
pub(crate) struct FieldName(String);

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
