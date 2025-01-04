#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetManagedDiskEncryptionSettingKeyEncryptionKey {
    /// The URL to the Key Vault Key used as the Key Encryption Key.
    #[builder(into)]
    #[serde(rename = "keyUrl")]
    pub r#key_url: Box<String>,
    /// The ID of the source Key Vault.
    #[builder(into)]
    #[serde(rename = "sourceVaultId")]
    pub r#source_vault_id: Box<String>,
}
