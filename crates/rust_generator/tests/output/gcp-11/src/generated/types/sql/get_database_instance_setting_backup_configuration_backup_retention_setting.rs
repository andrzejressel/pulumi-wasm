#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstanceSettingBackupConfigurationBackupRetentionSetting {
    /// Number of backups to retain.
    #[builder(into)]
    #[serde(rename = "retainedBackups")]
    pub r#retained_backups: Box<i32>,
    /// The unit that 'retainedBackups' represents. Defaults to COUNT
    #[builder(into)]
    #[serde(rename = "retentionUnit")]
    pub r#retention_unit: Box<String>,
}
