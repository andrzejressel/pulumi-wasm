#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge {
    #[builder(into, default)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridgeErrorHandlingConfig>>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
