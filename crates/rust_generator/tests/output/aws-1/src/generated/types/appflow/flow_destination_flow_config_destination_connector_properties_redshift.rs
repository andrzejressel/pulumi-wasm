#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift {
    #[builder(into, default)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesRedshiftErrorHandlingConfig>>,
    #[builder(into)]
    #[serde(rename = "intermediateBucketName")]
    pub r#intermediate_bucket_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
