#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRepositoryCreationTemplateEncryptionConfiguration {
    /// Encryption type to use for any created repositories, either `AES256` or `KMS`.
    #[builder(into)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: Box<String>,
    /// If `encryption_type` is `KMS`, the ARN of the KMS key used.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<String>,
}
