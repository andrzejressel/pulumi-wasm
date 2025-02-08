#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketReplicationConfigRuleSourceSelectionCriteria {
    /// Configuration block that you can specify for selections for modifications on replicas. Amazon S3 doesn't replicate replica modifications by default. In the latest version of replication configuration (when `filter` is specified), you can specify this element and set the status to `Enabled` to replicate modifications on replicas.
    #[builder(into, default)]
    #[serde(rename = "replicaModifications")]
    pub r#replica_modifications: Box<Option<super::super::types::s3::BucketReplicationConfigRuleSourceSelectionCriteriaReplicaModifications>>,
    /// Configuration block for filter information for the selection of Amazon S3 objects encrypted with AWS KMS. If specified, `replica_kms_key_id` in `destination` `encryption_configuration` must be specified as well.
    #[builder(into, default)]
    #[serde(rename = "sseKmsEncryptedObjects")]
    pub r#sse_kms_encrypted_objects: Box<Option<super::super::types::s3::BucketReplicationConfigRuleSourceSelectionCriteriaSseKmsEncryptedObjects>>,
}
