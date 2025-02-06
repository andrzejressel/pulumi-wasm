#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceSettingsBackupConfiguration {
    /// Backup retention settings. The configuration is detailed below.
    #[builder(into, default)]
    #[serde(rename = "backupRetentionSettings")]
    pub r#backup_retention_settings: Box<Option<super::super::types::sql::DatabaseInstanceSettingsBackupConfigurationBackupRetentionSettings>>,
    /// True if binary logging is enabled.
    /// Can only be used with MySQL.
    #[builder(into, default)]
    #[serde(rename = "binaryLogEnabled")]
    pub r#binary_log_enabled: Box<Option<bool>>,
    /// True if backup configuration is enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The region where the backup will be stored
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// True if Point-in-time recovery is enabled. Will restart database if enabled after instance creation. Valid only for PostgreSQL and SQL Server instances.
    #[builder(into, default)]
    #[serde(rename = "pointInTimeRecoveryEnabled")]
    pub r#point_in_time_recovery_enabled: Box<Option<bool>>,
    /// `HH:MM` format time indicating when backup
    /// configuration starts.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
    /// The number of days of transaction logs we retain for point in time restore, from 1-7. For PostgreSQL Enterprise Plus instances, the number of days of retained transaction logs can be set from 1 to 35.
    #[builder(into, default)]
    #[serde(rename = "transactionLogRetentionDays")]
    pub r#transaction_log_retention_days: Box<Option<i32>>,
}
