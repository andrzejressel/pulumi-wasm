#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityConfigurationEncryptionConfigurationS3Encryption {
    /// Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
    /// Encryption mode to use for S3 data. Valid values: `DISABLED`, `SSE-KMS`, `SSE-S3`. Default value: `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "s3EncryptionMode")]
    pub r#s_3_encryption_mode: Box<Option<String>>,
}
