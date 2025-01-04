#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterKeyManagementService {
    /// Identifier of Azure Key Vault key. See [key identifier format](https://learn.microsoft.com/en-us/azure/key-vault/general/about-keys-secrets-certificates#vault-name-and-object-name) for more details.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<String>,
    /// Network access of the key vault. The possible values are `Public` and `Private`. `Public` means the key vault allows public access from all networks. `Private` means the key vault disables public access and enables private link.
    #[builder(into)]
    #[serde(rename = "keyVaultNetworkAccess")]
    pub r#key_vault_network_access: Box<String>,
}
