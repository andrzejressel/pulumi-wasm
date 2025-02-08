#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketReplicationConfigurationRuleDestination {
    /// Specifies the overrides to use for object owners on replication. Must be used in conjunction with `account_id` owner override configuration.
    #[builder(into, default)]
    #[serde(rename = "accessControlTranslation")]
    pub r#access_control_translation: Box<Option<super::super::types::s3::BucketReplicationConfigurationRuleDestinationAccessControlTranslation>>,
    /// The Account ID to use for overriding the object owner on replication. Must be used in conjunction with `access_control_translation` override configuration.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// The ARN of the S3 bucket where you want Amazon S3 to store replicas of the object identified by the rule.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Enables replication metrics (required for S3 RTC) (documented below).
    #[builder(into, default)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Option<super::super::types::s3::BucketReplicationConfigurationRuleDestinationMetrics>>,
    /// Destination KMS encryption key ARN for SSE-KMS replication. Must be used in conjunction with
    /// `sse_kms_encrypted_objects` source selection criteria.
    #[builder(into, default)]
    #[serde(rename = "replicaKmsKeyId")]
    pub r#replica_kms_key_id: Box<Option<String>>,
    /// Enables S3 Replication Time Control (S3 RTC) (documented below).
    #[builder(into, default)]
    #[serde(rename = "replicationTime")]
    pub r#replication_time: Box<Option<super::super::types::s3::BucketReplicationConfigurationRuleDestinationReplicationTime>>,
    /// The [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_Destination.html#AmazonS3-Type-Destination-StorageClass) used to store the object. By default, Amazon S3 uses the storage class of the source object to create the object replica.
    #[builder(into, default)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Box<Option<String>>,
}
