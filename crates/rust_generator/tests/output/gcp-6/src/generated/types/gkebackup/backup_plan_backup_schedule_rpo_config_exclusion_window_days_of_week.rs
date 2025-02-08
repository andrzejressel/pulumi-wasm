#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPlanBackupScheduleRpoConfigExclusionWindowDaysOfWeek {
    /// A list of days of week.
    /// Each value may be one of: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into, default)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Option<Vec<String>>>,
}
