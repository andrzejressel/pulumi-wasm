#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxFunctionAppSlotBackupSchedule {
    /// How often the backup should be executed (e.g. for weekly backup, this should be set to `7` and `frequency_unit` should be set to `Day`).
    /// 
    /// > **NOTE:** Not all intervals are supported on all Linux Function App SKUs. Please refer to the official documentation for appropriate values.
    #[builder(into)]
    #[serde(rename = "frequencyInterval")]
    pub r#frequency_interval: Box<i32>,
    /// The unit of time for how often the backup should take place. Possible values include: `Day` and `Hour`.
    #[builder(into)]
    #[serde(rename = "frequencyUnit")]
    pub r#frequency_unit: Box<String>,
    /// Should the service keep at least one backup, regardless of age of backup. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "keepAtLeastOneBackup")]
    pub r#keep_at_least_one_backup: Box<Option<bool>>,
    /// The time the backup was last attempted.
    #[builder(into, default)]
    #[serde(rename = "lastExecutionTime")]
    pub r#last_execution_time: Box<Option<String>>,
    /// After how many days backups should be deleted. Defaults to `30`.
    #[builder(into, default)]
    #[serde(rename = "retentionPeriodDays")]
    pub r#retention_period_days: Box<Option<i32>>,
    /// When the schedule should start working in RFC-3339 format.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}
