#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceLoggingConfigurationAccessLogs {
    /// A block that specifies configures sending Verified Access logs to CloudWatch Logs. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogsCloudwatchLogs>>,
    /// Include trust data sent by trust providers into the logs.
    #[builder(into, default)]
    #[serde(rename = "includeTrustContext")]
    pub r#include_trust_context: Box<Option<bool>>,
    /// A block that specifies configures sending Verified Access logs to Kinesis. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "kinesisDataFirehose")]
    pub r#kinesis_data_firehose: Box<Option<super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogsKinesisDataFirehose>>,
    /// The logging version to use. Refer to [VerifiedAccessLogOptions](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_VerifiedAccessLogOptions.html) for the allowed values.
    #[builder(into, default)]
    #[serde(rename = "logVersion")]
    pub r#log_version: Box<Option<String>>,
    /// A block that specifies configures sending Verified Access logs to S3. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogsS3>>,
}
