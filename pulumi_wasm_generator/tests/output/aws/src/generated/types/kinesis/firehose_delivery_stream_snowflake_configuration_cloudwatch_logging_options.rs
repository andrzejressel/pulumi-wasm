#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamSnowflakeConfigurationCloudwatchLoggingOptions {
    /// Enables or disables the logging. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The CloudWatch group name for logging. This value is required if `enabled` is true.
    #[builder(into, default)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<Option<String>>,
    /// The CloudWatch log stream name for logging. This value is required if `enabled` is true.
    #[builder(into, default)]
    #[serde(rename = "logStreamName")]
    pub r#log_stream_name: Box<Option<String>>,
}