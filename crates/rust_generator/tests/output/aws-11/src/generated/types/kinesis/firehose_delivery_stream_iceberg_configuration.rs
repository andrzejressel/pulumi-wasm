#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamIcebergConfiguration {
    /// Buffer incoming data for the specified period of time, in seconds between 0 and 900, before delivering it to the destination. The default value is 300.
    #[builder(into, default)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Box<Option<i32>>,
    /// Buffer incoming data to the specified size, in MBs between 1 and 128, before delivering it to the destination. The default value is 5.
    #[builder(into, default)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Box<Option<i32>>,
    /// Glue catalog ARN identifier of the destination Apache Iceberg Tables. You must specify the ARN in the format `arn:aws:glue:region:account-id:catalog`
    #[builder(into)]
    #[serde(rename = "catalogArn")]
    pub r#catalog_arn: Box<String>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationCloudwatchLoggingOptions>>,
    /// Destination table configurations which Firehose uses to deliver data to Apache Iceberg Tables. Firehose will write data with insert if table specific configuration is not provided. See `destination_table_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "destinationTableConfigurations")]
    pub r#destination_table_configurations: Box<Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationDestinationTableConfiguration>>>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationProcessingConfiguration>>,
    /// The period of time, in seconds between 0 to 7200, during which Firehose retries to deliver data to the specified destination.
    #[builder(into, default)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Box<Option<i32>>,
    /// The ARN of the IAM role to be assumed by Firehose for calling Apache Iceberg Tables.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Box<Option<String>>,
    /// The S3 Configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationS3Configuration>,
}
