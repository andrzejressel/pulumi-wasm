#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamExtendedS3Configuration {
    /// The ARN of the S3 bucket
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationCloudwatchLoggingOptions>>,
    /// The compression format. If no value is specified, the default is `UNCOMPRESSED`. Other supported values are `GZIP`, `ZIP`, `Snappy`, & `HADOOP_SNAPPY`.
    #[builder(into, default)]
    #[serde(rename = "compressionFormat")]
    pub r#compression_format: Box<Option<String>>,
    /// The time zone you prefer. Valid values are `UTC` or a non-3-letter IANA time zones (for example, `America/Los_Angeles`). Default value is `UTC`.
    #[builder(into, default)]
    #[serde(rename = "customTimeZone")]
    pub r#custom_time_zone: Box<Option<String>>,
    /// Nested argument for the serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3. See `data_format_conversion_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "dataFormatConversionConfiguration")]
    pub r#data_format_conversion_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfiguration>>,
    /// The configuration for dynamic partitioning. Required when using [dynamic partitioning](https://docs.aws.amazon.com/firehose/latest/dev/dynamic-partitioning.html). See `dynamic_partitioning_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "dynamicPartitioningConfiguration")]
    pub r#dynamic_partitioning_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDynamicPartitioningConfiguration>>,
    /// Prefix added to failed records before writing them to S3. Not currently supported for `redshift` destination. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see [Custom Prefixes for Amazon S3 Objects](https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html).
    #[builder(into, default)]
    #[serde(rename = "errorOutputPrefix")]
    pub r#error_output_prefix: Box<Option<String>>,
    /// The file extension to override the default file extension (for example, `.json`).
    #[builder(into, default)]
    #[serde(rename = "fileExtension")]
    pub r#file_extension: Box<Option<String>>,
    /// Specifies the KMS key ARN the stream will use to encrypt data. If not set, no encryption will
    /// be used.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
    /// The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationProcessingConfiguration>>,
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The configuration for backup in Amazon S3. Required if `s3_backup_mode` is `Enabled`. Supports the same fields as `s3_configuration` object.
    #[builder(into, default)]
    #[serde(rename = "s3BackupConfiguration")]
    pub r#s_3_backup_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationS3BackupConfiguration>>,
    /// The Amazon S3 backup mode.  Valid values are `Disabled` and `Enabled`.  Default value is `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Box<Option<String>>,
}
