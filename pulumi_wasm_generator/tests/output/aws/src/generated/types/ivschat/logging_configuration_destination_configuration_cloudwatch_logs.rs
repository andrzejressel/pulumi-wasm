#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoggingConfigurationDestinationConfigurationCloudwatchLogs {
    /// Name of the Amazon Cloudwatch Logs destination where chat activity will be logged.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<String>,
}