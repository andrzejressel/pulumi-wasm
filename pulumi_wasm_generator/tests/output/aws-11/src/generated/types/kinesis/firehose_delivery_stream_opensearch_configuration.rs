#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamOpensearchConfiguration {
    /// Buffer incoming data for the specified period of time, in seconds between 0 to 900, before delivering it to the destination.  The default value is 300s.
    #[builder(into, default)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Box<Option<i32>>,
    /// Buffer incoming data to the specified size, in MBs between 1 to 100, before delivering it to the destination.  The default value is 5MB.
    #[builder(into, default)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Box<Option<i32>>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchConfigurationCloudwatchLoggingOptions>>,
    /// The endpoint to use when communicating with the cluster. Conflicts with `domain_arn`.
    #[builder(into, default)]
    #[serde(rename = "clusterEndpoint")]
    pub r#cluster_endpoint: Box<Option<String>>,
    /// The method for setting up document ID. See [`document_id_options` block] below for details.
    #[builder(into, default)]
    #[serde(rename = "documentIdOptions")]
    pub r#document_id_options: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchConfigurationDocumentIdOptions>>,
    /// The ARN of the Amazon ES domain.  The pattern needs to be `arn:.*`.  Conflicts with `cluster_endpoint`.
    #[builder(into, default)]
    #[serde(rename = "domainArn")]
    pub r#domain_arn: Box<Option<String>>,
    /// The OpenSearch index name.
    #[builder(into)]
    #[serde(rename = "indexName")]
    pub r#index_name: Box<String>,
    /// The OpenSearch index rotation period.  Index rotation appends a timestamp to the IndexName to facilitate expiration of old data.  Valid values are `NoRotation`, `OneHour`, `OneDay`, `OneWeek`, and `OneMonth`.  The default value is `OneDay`.
    #[builder(into, default)]
    #[serde(rename = "indexRotationPeriod")]
    pub r#index_rotation_period: Box<Option<String>>,
    /// The data processing configuration. See `processing_configuration` block below for details.
    #[builder(into, default)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchConfigurationProcessingConfiguration>>,
    /// After an initial failure to deliver to Amazon OpenSearch, the total amount of time, in seconds between 0 to 7200, during which Firehose re-attempts delivery (including the first attempt).  After this time has elapsed, the failed documents are written to Amazon S3.  The default value is 300s.  There will be no retry if the value is 0.
    #[builder(into, default)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Box<Option<i32>>,
    /// The ARN of the IAM role to be assumed by Firehose for calling the Amazon ES Configuration API and for indexing documents.  The IAM role must have permission for `DescribeDomain`, `DescribeDomains`, and `DescribeDomainConfig`.  The pattern needs to be `arn:.*`.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Defines how documents should be delivered to Amazon S3.  Valid values are `FailedDocumentsOnly` and `AllDocuments`.  Default value is `FailedDocumentsOnly`.
    #[builder(into, default)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Box<Option<String>>,
    /// The S3 Configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchConfigurationS3Configuration>,
    /// The Elasticsearch type name with maximum length of 100 characters. Types are deprecated in OpenSearch_1.1. TypeName must be empty.
    #[builder(into, default)]
    #[serde(rename = "typeName")]
    pub r#type_name: Box<Option<String>>,
    /// The VPC configuration for the delivery stream to connect to OpenSearch associated with the VPC. See `vpc_config` block below for details.
    #[builder(into, default)]
    #[serde(rename = "vpcConfig")]
    pub r#vpc_config: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchConfigurationVpcConfig>>,
}
