#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk {
    #[builder(into, default)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig>>,
    #[builder(into, default)]
    #[serde(rename = "idFieldNames")]
    pub r#id_field_names: Box<Option<Vec<String>>>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "writeOperationType")]
    pub r#write_operation_type: Box<Option<String>>,
}
