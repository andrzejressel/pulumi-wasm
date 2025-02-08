#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineAutoBackup {
    /// Enable or disable encryption for backups. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "encryptionEnabled")]
    pub r#encryption_enabled: Box<Option<bool>>,
    /// Encryption password to use. Must be specified when encryption is enabled.
    #[builder(into, default)]
    #[serde(rename = "encryptionPassword")]
    pub r#encryption_password: Box<Option<String>>,
    /// A `manual_schedule` block as documented below. When this block is present, the schedule type is set to `Manual`. Without this block, the schedule type is set to `Automated`.
    #[builder(into, default)]
    #[serde(rename = "manualSchedule")]
    pub r#manual_schedule: Box<Option<super::super::types::mssql::VirtualMachineAutoBackupManualSchedule>>,
    /// Retention period of backups, in days. Valid values are from `1` to `30`.
    #[builder(into)]
    #[serde(rename = "retentionPeriodInDays")]
    pub r#retention_period_in_days: Box<i32>,
    /// Access key for the storage account where backups will be kept.
    #[builder(into)]
    #[serde(rename = "storageAccountAccessKey")]
    pub r#storage_account_access_key: Box<String>,
    /// Blob endpoint for the storage account where backups will be kept.
    #[builder(into)]
    #[serde(rename = "storageBlobEndpoint")]
    pub r#storage_blob_endpoint: Box<String>,
    /// Include or exclude system databases from auto backup.
    #[builder(into, default)]
    #[serde(rename = "systemDatabasesBackupEnabled")]
    pub r#system_databases_backup_enabled: Box<Option<bool>>,
}
