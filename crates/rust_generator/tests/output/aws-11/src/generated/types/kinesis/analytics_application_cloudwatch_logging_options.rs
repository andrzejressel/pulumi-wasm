#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyticsApplicationCloudwatchLoggingOptions {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The ARN of the CloudWatch Log Stream.
    #[builder(into)]
    #[serde(rename = "logStreamArn")]
    pub r#log_stream_arn: Box<String>,
    /// The ARN of the IAM Role used to send application messages.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
