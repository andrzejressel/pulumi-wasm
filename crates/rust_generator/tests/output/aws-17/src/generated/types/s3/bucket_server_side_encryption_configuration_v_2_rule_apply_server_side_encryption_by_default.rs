#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketServerSideEncryptionConfigurationV2RuleApplyServerSideEncryptionByDefault {
    /// AWS KMS master key ID used for the SSE-KMS encryption. This can only be used when you set the value of `sse_algorithm` as `aws:kms`. The default `aws/s3` AWS KMS master key is used if this element is absent while the `sse_algorithm` is `aws:kms`.
    #[builder(into, default)]
    #[serde(rename = "kmsMasterKeyId")]
    pub r#kms_master_key_id: Box<Option<String>>,
    /// Server-side encryption algorithm to use. Valid values are `AES256`, `aws:kms`, and `aws:kms:dsse`
    #[builder(into)]
    #[serde(rename = "sseAlgorithm")]
    pub r#sse_algorithm: Box<String>,
}
