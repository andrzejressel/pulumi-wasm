#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicatedVmManagedDiskTargetDiskEncryptionDiskEncryptionKey {
    /// The URL to the Key Vault Secret used as the Disk Encryption Key that the Managed Disk will be associated with. This can be found as `id` on the `azure.keyvault.Secret` resource. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "secretUrl")]
    pub r#secret_url: Box<String>,
    /// The ID of the Key Vault. This can be found as `id` on the `azure.keyvault.KeyVault` resource. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vaultId")]
    pub r#vault_id: Box<String>,
}
