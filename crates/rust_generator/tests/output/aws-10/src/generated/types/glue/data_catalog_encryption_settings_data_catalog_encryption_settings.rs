#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataCatalogEncryptionSettingsDataCatalogEncryptionSettings {
    /// When connection password protection is enabled, the Data Catalog uses a customer-provided key to encrypt the password as part of CreateConnection or UpdateConnection and store it in the ENCRYPTED_PASSWORD field in the connection properties. You can enable catalog encryption or only password encryption. see Connection Password Encryption.
    #[builder(into)]
    #[serde(rename = "connectionPasswordEncryption")]
    pub r#connection_password_encryption: Box<super::super::types::glue::DataCatalogEncryptionSettingsDataCatalogEncryptionSettingsConnectionPasswordEncryption>,
    /// Specifies the encryption-at-rest configuration for the Data Catalog. see Encryption At Rest.
    #[builder(into)]
    #[serde(rename = "encryptionAtRest")]
    pub r#encryption_at_rest: Box<super::super::types::glue::DataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEncryptionAtRest>,
}
