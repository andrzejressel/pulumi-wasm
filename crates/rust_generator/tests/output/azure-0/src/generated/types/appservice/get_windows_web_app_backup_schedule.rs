#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsWebAppBackupSchedule {
    /// How often the backup will be executed.
    #[builder(into)]
    #[serde(rename = "frequencyInterval")]
    pub r#frequency_interval: Box<i32>,
    /// The unit of time for how often the backup should take place.
    #[builder(into)]
    #[serde(rename = "frequencyUnit")]
    pub r#frequency_unit: Box<String>,
    /// Will the service keep at least one backup, regardless of age of backup.
    #[builder(into)]
    #[serde(rename = "keepAtLeastOneBackup")]
    pub r#keep_at_least_one_backup: Box<bool>,
    /// The time of the last backup attempt.
    #[builder(into)]
    #[serde(rename = "lastExecutionTime")]
    pub r#last_execution_time: Box<String>,
    /// After how many days backups should be deleted.
    #[builder(into)]
    #[serde(rename = "retentionPeriodDays")]
    pub r#retention_period_days: Box<i32>,
    /// When the schedule should start in RFC-3339 format.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
