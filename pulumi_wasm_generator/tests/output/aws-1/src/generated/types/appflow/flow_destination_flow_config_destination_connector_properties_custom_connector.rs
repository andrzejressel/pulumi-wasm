#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector {
    #[builder(into, default)]
    #[serde(rename = "customProperties")]
    pub r#custom_properties: Box<Option<std::collections::HashMap<String, String>>>,
    #[builder(into)]
    #[serde(rename = "entityName")]
    pub r#entity_name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig>>,
    #[builder(into, default)]
    #[serde(rename = "idFieldNames")]
    pub r#id_field_names: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "writeOperationType")]
    pub r#write_operation_type: Box<Option<String>>,
}
