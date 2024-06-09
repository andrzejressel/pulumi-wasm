#[cfg(test)]
use mockall::automock;
use rmpv::Value;
use std::collections::HashMap;
use crate::node::NodeValue;

#[cfg_attr(test, automock)]
pub(crate) trait Pulumi {
    fn is_in_preview(&self) -> bool;
    fn get_root_resource(&self) -> String;
    fn register_outputs(&self, outputs: HashMap<String, Value>);
    fn register_resource(&self, request: RegisterResourceRequest) -> RegisterResourceResponse;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FieldName(String);

impl FieldName {
    pub(crate) fn new(s: String) -> Self {
        Self(s)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterResourceRequest {
    pub(crate) r#type: String,
    pub(crate) name: String,
    pub(crate) object: HashMap<FieldName, Value>,
    pub(crate) expected_results: HashMap<FieldName, msgpack_protobuf_converter::Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterResourceResponse {
    outputs: HashMap<FieldName, NodeValue>,
}
