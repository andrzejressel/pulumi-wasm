#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData {
    #[builder(into, default)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataErrorHandlingConfig>>,
    #[builder(into, default)]
    #[serde(rename = "idFieldNames")]
    pub r#id_field_names: Box<Option<Vec<String>>>,
    #[builder(into)]
    #[serde(rename = "objectPath")]
    pub r#object_path: Box<String>,
    /// Determines how Amazon AppFlow handles the success response that it gets from the connector after placing data. See Success Response Handling Config for more details.
    #[builder(into, default)]
    #[serde(rename = "successResponseHandlingConfig")]
    pub r#success_response_handling_config: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfig>>,
    #[builder(into, default)]
    #[serde(rename = "writeOperationType")]
    pub r#write_operation_type: Box<Option<String>>,
}
