#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityConfigurationEncryptionConfigurationJobBookmarksEncryption {
    /// Encryption mode to use for job bookmarks data. Valid values: `CSE-KMS`, `DISABLED`. Default value: `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "jobBookmarksEncryptionMode")]
    pub r#job_bookmarks_encryption_mode: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
}
