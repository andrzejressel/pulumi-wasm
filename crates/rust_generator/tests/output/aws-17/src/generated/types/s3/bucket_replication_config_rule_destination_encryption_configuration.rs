#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketReplicationConfigRuleDestinationEncryptionConfiguration {
    /// ID (Key ARN or Alias ARN) of the customer managed AWS KMS key stored in AWS Key Management Service (KMS) for the destination bucket.
    #[builder(into)]
    #[serde(rename = "replicaKmsKeyId")]
    pub r#replica_kms_key_id: Box<String>,
}
