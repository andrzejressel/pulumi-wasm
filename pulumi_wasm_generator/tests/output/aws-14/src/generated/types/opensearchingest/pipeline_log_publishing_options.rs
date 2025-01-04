#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineLogPublishingOptions {
    /// The destination for OpenSearch Ingestion logs sent to Amazon CloudWatch Logs. This parameter is required if IsLoggingEnabled is set to true. See `cloudwatch_log_destination` below.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogDestination")]
    pub r#cloudwatch_log_destination: Box<Option<super::super::types::opensearchingest::PipelineLogPublishingOptionsCloudwatchLogDestination>>,
    /// Whether logs should be published.
    #[builder(into, default)]
    #[serde(rename = "isLoggingEnabled")]
    pub r#is_logging_enabled: Box<Option<bool>>,
}
