use crate::model::{FieldName, OutputId};
#[cfg(test)]
use mockall::automock;
use rmpv::Value;
use std::collections::{HashMap, HashSet};

#[cfg_attr(test, automock)]
pub trait PulumiService {
    fn is_in_preview(&self) -> bool;
    fn get_root_resource(&self) -> String;
    fn register_outputs(&self, outputs: HashMap<FieldName, Value>);
    fn register_resource(&self, output_id: OutputId, request: RegisterResourceRequest);
    fn register_resource_poll(
        &self,
        register_ids: &HashSet<OutputId>,
    ) -> HashMap<OutputId, RegisterResourceResponse>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterResourceRequest {
    pub(crate) r#type: String,
    pub(crate) name: String,
    pub(crate) object: HashMap<FieldName, Option<Value>>,
    pub(crate) expected_results: HashMap<FieldName, msgpack_protobuf_converter::Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterResourceResponse {
    pub(crate) outputs: HashMap<FieldName, Value>,
}
