#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CanaryArtifactConfigS3Encryption {
    /// The encryption method to use for artifacts created by this canary. Valid values are: `SSE_S3` and `SSE_KMS`.
    #[builder(into, default)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: Box<Option<String>>,
    /// The ARN of the customer-managed KMS key to use, if you specify `SSE_KMS` for `encryption_mode`.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
}
