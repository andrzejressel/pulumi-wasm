#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetManagedDiskEncryptionSettingDiskEncryptionKey {
    /// The URL to the Key Vault Secret used as the Disk Encryption Key.
    #[builder(into)]
    #[serde(rename = "secretUrl")]
    pub r#secret_url: Box<String>,
    /// The ID of the source Key Vault.
    #[builder(into)]
    #[serde(rename = "sourceVaultId")]
    pub r#source_vault_id: Box<String>,
}
