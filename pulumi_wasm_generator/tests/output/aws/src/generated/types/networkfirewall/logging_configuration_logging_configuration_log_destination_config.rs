#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoggingConfigurationLoggingConfigurationLogDestinationConfig {
    /// A map describing the logging destination for the chosen `log_destination_type`.
    /// * For an Amazon S3 bucket, specify the key `bucketName` with the name of the bucket and optionally specify the key `prefix` with a path.
    /// * For a CloudWatch log group, specify the key `logGroup` with the name of the CloudWatch log group.
    /// * For a Kinesis Data Firehose delivery stream, specify the key `deliveryStream` with the name of the delivery stream.
    #[builder(into)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: Box<std::collections::HashMap<String, String>>,
    /// The location to send logs to. Valid values: `S3`, `CloudWatchLogs`, `KinesisDataFirehose`.
    #[builder(into)]
    #[serde(rename = "logDestinationType")]
    pub r#log_destination_type: Box<String>,
    /// The type of log to send. Valid values: `ALERT` or `FLOW` or `TLS`. Alert logs report traffic that matches a `StatefulRule` with an action setting that sends a log message. Flow logs are standard network traffic flow logs.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
}
