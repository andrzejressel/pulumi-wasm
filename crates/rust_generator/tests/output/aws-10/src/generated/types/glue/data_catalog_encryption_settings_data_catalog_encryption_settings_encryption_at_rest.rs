#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEncryptionAtRest {
    /// The encryption-at-rest mode for encrypting Data Catalog data. Valid values: `DISABLED`, `SSE-KMS`, `SSE-KMS-WITH-SERVICE-ROLE`.
    #[builder(into)]
    #[serde(rename = "catalogEncryptionMode")]
    pub r#catalog_encryption_mode: Box<String>,
    /// The ARN of the AWS IAM role used for accessing encrypted Data Catalog data.
    #[builder(into, default)]
    #[serde(rename = "catalogEncryptionServiceRole")]
    pub r#catalog_encryption_service_role: Box<Option<String>>,
    /// The ARN of the AWS KMS key to use for encryption at rest.
    #[builder(into, default)]
    #[serde(rename = "sseAwsKmsKeyId")]
    pub r#sse_aws_kms_key_id: Box<Option<String>>,
}
