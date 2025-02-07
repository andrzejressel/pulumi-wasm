#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstanceSettingBackupConfiguration {
    #[builder(into)]
    #[serde(rename = "backupRetentionSettings")]
    pub r#backup_retention_settings: Box<Vec<super::super::types::sql::GetDatabaseInstanceSettingBackupConfigurationBackupRetentionSetting>>,
    /// True if binary logging is enabled. If settings.backup_configuration.enabled is false, this must be as well. Can only be used with MySQL.
    #[builder(into)]
    #[serde(rename = "binaryLogEnabled")]
    pub r#binary_log_enabled: Box<bool>,
    /// True if backup configuration is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Location of the backup configuration.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// True if Point-in-time recovery is enabled.
    #[builder(into)]
    #[serde(rename = "pointInTimeRecoveryEnabled")]
    pub r#point_in_time_recovery_enabled: Box<bool>,
    /// HH:MM format time indicating when backup configuration starts.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
    /// The number of days of transaction logs we retain for point in time restore, from 1-7. (For PostgreSQL Enterprise Plus instances, from 1 to 35.)
    #[builder(into)]
    #[serde(rename = "transactionLogRetentionDays")]
    pub r#transaction_log_retention_days: Box<i32>,
}
