#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeaturesNetapp {
    /// When enabled, backups will be deleted when the `azure.netapp.BackupVault` resource is destroyed
    #[builder(into, default)]
    #[serde(rename = "deleteBackupsOnBackupVaultDestroy")]
    pub r#delete_backups_on_backup_vault_destroy: Box<Option<bool>>,
    /// When enabled, the volume will not be destroyed, safeguarding from severe data loss
    #[builder(into, default)]
    #[serde(rename = "preventVolumeDestruction")]
    pub r#prevent_volume_destruction: Box<Option<bool>>,
}
