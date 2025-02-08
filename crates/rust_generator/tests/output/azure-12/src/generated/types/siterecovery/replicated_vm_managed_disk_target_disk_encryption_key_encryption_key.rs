#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ReplicatedVmManagedDiskTargetDiskEncryptionKeyEncryptionKey {
    /// The URL to the Key Vault Key used as the Key Encryption Key that the Managed Disk will be associated with. This can be found as `id` on the `azure.keyvault.Key` resource. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "keyUrl")]
    pub r#key_url: Box<String>,
    /// The ID of the Key Vault. This can be found as `id` on the `azure.keyvault.KeyVault` resource. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vaultId")]
    pub r#vault_id: Box<String>,
}
