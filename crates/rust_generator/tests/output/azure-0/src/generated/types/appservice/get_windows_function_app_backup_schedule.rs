#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsFunctionAppBackupSchedule {
    /// How often the backup is executed.
    #[builder(into)]
    #[serde(rename = "frequencyInterval")]
    pub r#frequency_interval: Box<i32>,
    /// The unit of time the backup should take place.
    #[builder(into)]
    #[serde(rename = "frequencyUnit")]
    pub r#frequency_unit: Box<String>,
    /// Should the service keep at least one backup.
    #[builder(into)]
    #[serde(rename = "keepAtLeastOneBackup")]
    pub r#keep_at_least_one_backup: Box<bool>,
    /// The time the backup was last attempted.
    #[builder(into)]
    #[serde(rename = "lastExecutionTime")]
    pub r#last_execution_time: Box<String>,
    /// After how many days backups is deleted.
    #[builder(into)]
    #[serde(rename = "retentionPeriodDays")]
    pub r#retention_period_days: Box<i32>,
    /// When the schedule should start working in RFC-3339 format.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
