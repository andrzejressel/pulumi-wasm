#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamRedshiftConfiguration {
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationCloudwatchLoggingOptions>>,
    /// The jdbcurl of the redshift cluster.
    #[builder(into)]
    #[serde(rename = "clusterJdbcurl")]
    pub r#cluster_jdbcurl: Box<String>,
    /// Copy options for copying the data from the s3 intermediate bucket into redshift, for example to change the default delimiter. For valid values, see the [AWS documentation](http://docs.aws.amazon.com/firehose/latest/APIReference/API_CopyCommand.html)
    #[builder(into, default)]
    #[serde(rename = "copyOptions")]
    pub r#copy_options: Box<Option<String>>,
    /// The data table columns that will be targeted by the copy command.
    #[builder(into, default)]
    #[serde(rename = "dataTableColumns")]
    pub r#data_table_columns: Box<Option<String>>,
    /// The name of the table in the redshift cluster that the s3 bucket will copy to.
    #[builder(into)]
    #[serde(rename = "dataTableName")]
    pub r#data_table_name: Box<String>,
    /// The password for the username above. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationProcessingConfiguration>>,
    /// The length of time during which Firehose retries delivery after a failure, starting from the initial request and including the first attempt. The default value is 3600 seconds (60 minutes). Firehose does not retry if the value of DurationInSeconds is 0 (zero) or if the first delivery attempt takes longer than the current value.
    #[builder(into, default)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Box<Option<i32>>,
    /// The arn of the role the stream assumes.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The configuration for backup in Amazon S3. Required if `s3_backup_mode` is `Enabled`. Supports the same fields as `s3_configuration` object.
    /// `secrets_manager_configuration` - (Optional) The Secrets Manager configuration. See `secrets_manager_configuration` block below for details. This value is required if `username` and `password` are not provided.
    #[builder(into, default)]
    #[serde(rename = "s3BackupConfiguration")]
    pub r#s_3_backup_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationS3BackupConfiguration>>,
    /// The Amazon S3 backup mode.  Valid values are `Disabled` and `Enabled`.  Default value is `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Box<Option<String>>,
    /// The S3 Configuration. See s3_configuration below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationS3Configuration>,
    #[builder(into, default)]
    #[serde(rename = "secretsManagerConfiguration")]
    pub r#secrets_manager_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationSecretsManagerConfiguration>>,
    /// The username that the firehose delivery stream will assume. It is strongly recommended that the username and password provided is used exclusively for Amazon Kinesis Firehose purposes, and that the permissions for the account are restricted for Amazon Redshift INSERT permissions. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
