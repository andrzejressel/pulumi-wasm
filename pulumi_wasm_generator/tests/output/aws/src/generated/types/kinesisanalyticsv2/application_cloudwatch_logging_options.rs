#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationCloudwatchLoggingOptions {
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLoggingOptionId")]
    pub r#cloudwatch_logging_option_id: Box<Option<String>>,
    /// The ARN of the CloudWatch log stream to receive application messages.
    #[builder(into)]
    #[serde(rename = "logStreamArn")]
    pub r#log_stream_arn: Box<String>,
}