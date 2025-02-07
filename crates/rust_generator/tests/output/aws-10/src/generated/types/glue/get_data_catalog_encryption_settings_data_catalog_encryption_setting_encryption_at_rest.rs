#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCatalogEncryptionSettingsDataCatalogEncryptionSettingEncryptionAtRest {
    /// The encryption-at-rest mode for encrypting Data Catalog data.
    #[builder(into)]
    #[serde(rename = "catalogEncryptionMode")]
    pub r#catalog_encryption_mode: Box<String>,
    /// The ARN of the AWS IAM role used for accessing encrypted Data Catalog data.
    #[builder(into)]
    #[serde(rename = "catalogEncryptionServiceRole")]
    pub r#catalog_encryption_service_role: Box<String>,
    /// ARN of the AWS KMS key to use for encryption at rest.
    #[builder(into)]
    #[serde(rename = "sseAwsKmsKeyId")]
    pub r#sse_aws_kms_key_id: Box<String>,
}
