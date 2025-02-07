#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppServiceBackupSchedule {
    /// Sets how often the backup should be executed.
    #[builder(into)]
    #[serde(rename = "frequencyInterval")]
    pub r#frequency_interval: Box<i32>,
    /// Sets the unit of time for how often the backup should be executed. Possible values are `Day` or `Hour`.
    #[builder(into)]
    #[serde(rename = "frequencyUnit")]
    pub r#frequency_unit: Box<String>,
    /// Should at least one backup always be kept in the Storage Account by the Retention Policy, regardless of how old it is?
    #[builder(into, default)]
    #[serde(rename = "keepAtLeastOneBackup")]
    pub r#keep_at_least_one_backup: Box<Option<bool>>,
    /// Specifies the number of days after which Backups should be deleted. Defaults to `30`.
    #[builder(into, default)]
    #[serde(rename = "retentionPeriodInDays")]
    pub r#retention_period_in_days: Box<Option<i32>>,
    /// Sets when the schedule should start working.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}
