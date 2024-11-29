use crate::model::{FieldName, OutputId};
use crate::nodes::ResourceRequestOperation;
#[cfg(test)]
use mockall::automock;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[cfg_attr(test, automock)]
pub trait PulumiService {
    fn is_in_preview(&self) -> bool;
    fn get_root_resource(&self) -> String;
    fn register_outputs(&self, outputs: HashMap<FieldName, Value>);
    fn perform_resource_operation(&self, output_id: OutputId, request: PerformResourceRequest);
    fn poll_resource_operations(
        &self,
        register_ids: &HashSet<OutputId>,
    ) -> HashMap<OutputId, RegisterResourceResponse>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct PerformResourceRequest {
    pub(crate) operation: ResourceRequestOperation,
    pub(crate) object: HashMap<FieldName, Option<Value>>,
    pub(crate) expected_results: HashSet<FieldName>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterResourceResponse {
    pub(crate) outputs: HashMap<FieldName, Value>,
}
