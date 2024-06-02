use crate::model::FieldName;
use rmpv::Value;
use std::collections::HashMap;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub(crate) trait Pulumi {
    fn is_in_preview(&self) -> bool;
    fn get_root_resource(&self) -> String;
    fn register_outputs(&self, outputs: Vec<(String, Value)>);
    fn register_resource(&self, request: RegisterResourceRequest) -> RegisterResourceResponse;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterResourceRequest {
    r#type: String,
    name: String,
    object: Vec<(Value, Value)>,
    expected_results: HashMap<FieldName, msgpack_protobuf_converter::Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RegisterResourceResponse {
    outputs: Value,
}
