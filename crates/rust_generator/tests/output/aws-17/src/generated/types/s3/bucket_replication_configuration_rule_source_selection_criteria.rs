#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketReplicationConfigurationRuleSourceSelectionCriteria {
    /// Match SSE-KMS encrypted objects (documented below). If specified, `replica_kms_key_id`
    /// in `destination` must be specified as well.
    #[builder(into, default)]
    #[serde(rename = "sseKmsEncryptedObjects")]
    pub r#sse_kms_encrypted_objects: Box<Option<super::super::types::s3::BucketReplicationConfigurationRuleSourceSelectionCriteriaSseKmsEncryptedObjects>>,
}
