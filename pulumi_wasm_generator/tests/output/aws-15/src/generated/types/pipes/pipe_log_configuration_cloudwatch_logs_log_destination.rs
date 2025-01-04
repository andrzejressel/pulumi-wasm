#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeLogConfigurationCloudwatchLogsLogDestination {
    /// Amazon Web Services Resource Name (ARN) for the CloudWatch log group to which EventBridge sends the log records.
    #[builder(into)]
    #[serde(rename = "logGroupArn")]
    pub r#log_group_arn: Box<String>,
}
