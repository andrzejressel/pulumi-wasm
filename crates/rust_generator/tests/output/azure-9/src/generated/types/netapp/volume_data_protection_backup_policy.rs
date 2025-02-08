#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VolumeDataProtectionBackupPolicy {
    /// Resource ID of the backup policy to apply to the volume.
    #[builder(into)]
    #[serde(rename = "backupPolicyId")]
    pub r#backup_policy_id: Box<String>,
    /// Resource ID of the backup backup vault to associate this volume to.
    #[builder(into)]
    #[serde(rename = "backupVaultId")]
    pub r#backup_vault_id: Box<String>,
    /// Enables the backup policy on the volume, defaults to `true`.
    /// 
    /// For more information on Azure NetApp Files Backup feature please see [Understand Azure NetApp Files backup](https://learn.microsoft.com/en-us/azure/azure-netapp-files/backup-introduction)
    #[builder(into, default)]
    #[serde(rename = "policyEnabled")]
    pub r#policy_enabled: Box<Option<bool>>,
}
