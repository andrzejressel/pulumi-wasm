#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesSapoData {
    #[builder(into)]
    #[serde(rename = "objectPath")]
    pub r#object_path: Box<String>,
    /// Sets the page size for each concurrent process that transfers OData records from your SAP instance.
    #[builder(into, default)]
    #[serde(rename = "paginationConfig")]
    pub r#pagination_config: Box<Option<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig>>,
    /// Sets the number of concurrent processes that transfers OData records from your SAP instance.
    #[builder(into, default)]
    #[serde(rename = "parallelismConfig")]
    pub r#parallelism_config: Box<Option<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig>>,
}