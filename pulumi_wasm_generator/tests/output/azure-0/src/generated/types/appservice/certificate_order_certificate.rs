#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateOrderCertificate {
    /// The name of the App Service Certificate.
    #[builder(into, default)]
    #[serde(rename = "certificateName")]
    pub r#certificate_name: Box<Option<String>>,
    /// Key Vault resource Id.
    #[builder(into, default)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<Option<String>>,
    /// Key Vault secret name.
    #[builder(into, default)]
    #[serde(rename = "keyVaultSecretName")]
    pub r#key_vault_secret_name: Box<Option<String>>,
    /// Status of the Key Vault secret.
    #[builder(into, default)]
    #[serde(rename = "provisioningState")]
    pub r#provisioning_state: Box<Option<String>>,
}
