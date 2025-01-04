#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamSnowflakeConfiguration {
    /// The URL of the Snowflake account. Format: https://[account_identifier].snowflakecomputing.com.
    #[builder(into)]
    #[serde(rename = "accountUrl")]
    pub r#account_url: Box<String>,
    /// Buffer incoming data for the specified period of time, in seconds between 0 to 900, before delivering it to the destination.  The default value is 0s.
    #[builder(into, default)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Box<Option<i32>>,
    /// Buffer incoming data to the specified size, in MBs between 1 to 128, before delivering it to the destination.  The default value is 1MB.
    #[builder(into, default)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Box<Option<i32>>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationCloudwatchLoggingOptions>>,
    /// The name of the content column.
    #[builder(into, default)]
    #[serde(rename = "contentColumnName")]
    pub r#content_column_name: Box<Option<String>>,
    /// The data loading option.
    #[builder(into, default)]
    #[serde(rename = "dataLoadingOption")]
    pub r#data_loading_option: Box<Option<String>>,
    /// The Snowflake database name.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// The passphrase for the private key.
    #[builder(into, default)]
    #[serde(rename = "keyPassphrase")]
    pub r#key_passphrase: Box<Option<String>>,
    /// The name of the metadata column.
    #[builder(into, default)]
    #[serde(rename = "metadataColumnName")]
    pub r#metadata_column_name: Box<Option<String>>,
    /// The private key for authentication. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into, default)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    /// The processing configuration. See `processing_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationProcessingConfiguration>>,
    /// After an initial failure to deliver to Snowflake, the total amount of time, in seconds between 0 to 7200, during which Firehose re-attempts delivery (including the first attempt).  After this time has elapsed, the failed documents are written to Amazon S3.  The default value is 60s.  There will be no retry if the value is 0.
    #[builder(into, default)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Box<Option<i32>>,
    /// The ARN of the IAM role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The S3 backup mode.
    #[builder(into, default)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Box<Option<String>>,
    /// The S3 configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationS3Configuration>,
    /// The Snowflake schema name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<String>,
    /// The Secrets Manager configuration. See `secrets_manager_configuration` block below for details. This value is required if `user` and `private_key` are not provided.
    #[builder(into, default)]
    #[serde(rename = "secretsManagerConfiguration")]
    pub r#secrets_manager_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationSecretsManagerConfiguration>>,
    /// The configuration for Snowflake role.
    #[builder(into, default)]
    #[serde(rename = "snowflakeRoleConfiguration")]
    pub r#snowflake_role_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationSnowflakeRoleConfiguration>>,
    /// The VPC configuration for Snowflake.
    #[builder(into, default)]
    #[serde(rename = "snowflakeVpcConfiguration")]
    pub r#snowflake_vpc_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationSnowflakeVpcConfiguration>>,
    /// The Snowflake table name.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: Box<String>,
    /// The user for authentication. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into, default)]
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}
