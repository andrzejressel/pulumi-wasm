#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCatalogEncryptionSettingsDataCatalogEncryptionSettingConnectionPasswordEncryption {
    /// KMS key ARN that is used to encrypt the connection password.
    #[builder(into)]
    #[serde(rename = "awsKmsKeyId")]
    pub r#aws_kms_key_id: Box<String>,
    /// When set to `true`, passwords remain encrypted in the responses of GetConnection and GetConnections. This encryption takes effect independently of the catalog encryption.
    #[builder(into)]
    #[serde(rename = "returnConnectionPasswordEncrypted")]
    pub r#return_connection_password_encrypted: Box<bool>,
}