use crate::model::{FieldName, OutputId};
#[cfg(test)]
use mockall::automock;
use rmpv::Value;
use std::collections::{HashMap, HashSet};

#[cfg_attr(test, automock)]
pub trait Pulumi {
    fn is_in_preview(&self) -> bool;
    fn get_root_resource(&self) -> String;
    fn register_outputs(&self, outputs: HashMap<String, Value>);
    fn register_resource(&self, output_id: OutputId, request: RegisterResourceRequest);
    fn register_resource_poll(
        &self,
        register_ids: &HashSet<OutputId>,
    ) -> HashMap<OutputId, RegisterResourceResponse>;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RegisterId(String);

impl RegisterId {
    pub(crate) fn new(s: String) -> Self {
        Self(s)
    }
}
//
// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub struct FieldName(String);
//
// impl From<String> for FieldName {
//     fn from(s: String) -> Self {
//         Self(s)
//     }
// }
//
// impl From<&str> for FieldName {
//     fn from(s: &str) -> Self {
//         Self(s.to_string())
//     }
// }
//
// impl FieldName {
//     pub(crate) fn new(s: String) -> Self {
//         Self(s)
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterResourceRequest {
    pub(crate) r#type: String,
    pub(crate) name: String,
    pub(crate) object: HashMap<FieldName, Value>,
    pub(crate) expected_results: HashMap<FieldName, msgpack_protobuf_converter::Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterResourceResponse {
    pub(crate) outputs: HashMap<FieldName, Value>,
}
