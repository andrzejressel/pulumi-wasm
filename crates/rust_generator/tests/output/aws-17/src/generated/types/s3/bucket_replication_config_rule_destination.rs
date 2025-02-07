#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketReplicationConfigRuleDestination {
    /// Configuration block that specifies the overrides to use for object owners on replication. See below. Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the AWS account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same AWS account that owns the source object. Must be used in conjunction with `account` owner override configuration.
    #[builder(into, default)]
    #[serde(rename = "accessControlTranslation")]
    pub r#access_control_translation: Box<Option<super::super::types::s3::BucketReplicationConfigRuleDestinationAccessControlTranslation>>,
    /// Account ID to specify the replica ownership. Must be used in conjunction with `access_control_translation` override configuration.
    #[builder(into, default)]
    #[serde(rename = "account")]
    pub r#account: Box<Option<String>>,
    /// ARN of the bucket where you want Amazon S3 to store the results.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Configuration block that provides information about encryption. See below. If `source_selection_criteria` is specified, you must specify this element.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfiguration")]
    pub r#encryption_configuration: Box<Option<super::super::types::s3::BucketReplicationConfigRuleDestinationEncryptionConfiguration>>,
    /// Configuration block that specifies replication metrics-related settings enabling replication metrics and events. See below.
    #[builder(into, default)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Option<super::super::types::s3::BucketReplicationConfigRuleDestinationMetrics>>,
    /// Configuration block that specifies S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time when all objects and operations on objects must be replicated. See below. Replication Time Control must be used in conjunction with `metrics`.
    #[builder(into, default)]
    #[serde(rename = "replicationTime")]
    pub r#replication_time: Box<Option<super::super::types::s3::BucketReplicationConfigRuleDestinationReplicationTime>>,
    /// The [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_Destination.html#AmazonS3-Type-Destination-StorageClass) used to store the object. By default, Amazon S3 uses the storage class of the source object to create the object replica.
    #[builder(into, default)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Box<Option<String>>,
}
