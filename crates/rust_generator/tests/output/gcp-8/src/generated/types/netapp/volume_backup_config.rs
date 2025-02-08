#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeBackupConfig {
    /// Specify a single backup policy ID for scheduled backups. Format: `projects/{{projectId}}/locations/{{location}}/backupPolicies/{{backupPolicyName}}`
    #[builder(into, default)]
    #[serde(rename = "backupPolicies")]
    pub r#backup_policies: Box<Option<Vec<String>>>,
    /// ID of the backup vault to use. A backup vault is reqired to create manual or scheduled backups.
    /// Format: `projects/{{projectId}}/locations/{{location}}/backupVaults/{{backupVaultName}}`
    #[builder(into, default)]
    #[serde(rename = "backupVault")]
    pub r#backup_vault: Box<Option<String>>,
    /// When set to true, scheduled backup is enabled on the volume. Omit if no backup_policy is specified.
    #[builder(into, default)]
    #[serde(rename = "scheduledBackupEnabled")]
    pub r#scheduled_backup_enabled: Box<Option<bool>>,
}
