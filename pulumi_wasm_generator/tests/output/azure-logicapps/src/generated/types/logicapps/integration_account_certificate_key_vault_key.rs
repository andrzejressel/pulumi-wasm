#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationAccountCertificateKeyVaultKey {
    /// The name of Key Vault Key.
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: Box<String>,
    /// The ID of the Key Vault.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<String>,
    /// The version of Key Vault Key.
    #[builder(into, default)]
    #[serde(rename = "keyVersion")]
    pub r#key_version: Box<Option<String>>,
}