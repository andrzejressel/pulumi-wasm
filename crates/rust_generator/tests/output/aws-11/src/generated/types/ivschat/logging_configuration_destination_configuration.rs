#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoggingConfigurationDestinationConfiguration {
    /// An Amazon CloudWatch Logs destination configuration where chat activity will be logged.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::ivschat::LoggingConfigurationDestinationConfigurationCloudwatchLogs>>,
    /// An Amazon Kinesis Data Firehose destination configuration where chat activity will be logged.
    #[builder(into, default)]
    #[serde(rename = "firehose")]
    pub r#firehose: Box<Option<super::super::types::ivschat::LoggingConfigurationDestinationConfigurationFirehose>>,
    /// An Amazon S3 destination configuration where chat activity will be logged.
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::ivschat::LoggingConfigurationDestinationConfigurationS3>>,
}
