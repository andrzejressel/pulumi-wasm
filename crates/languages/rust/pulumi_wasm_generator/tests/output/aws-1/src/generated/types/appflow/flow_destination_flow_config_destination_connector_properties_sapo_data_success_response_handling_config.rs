#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesSapoDataSuccessResponseHandlingConfig {
    /// Name of the Amazon S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    /// Amazon S3 bucket prefix.
    #[builder(into, default)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<Option<String>>,
}
