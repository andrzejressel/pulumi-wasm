#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPlanBackupScheduleRpoConfigExclusionWindow {
    /// The exclusion window occurs every day if set to "True".
    /// Specifying this field to "False" is an error.
    /// Only one of singleOccurrenceDate, daily and daysOfWeek may be set.
    #[builder(into, default)]
    #[serde(rename = "daily")]
    pub r#daily: Box<Option<bool>>,
    /// The exclusion window occurs on these days of each week in UTC.
    /// Only one of singleOccurrenceDate, daily and daysOfWeek may be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "daysOfWeek")]
    pub r#days_of_week: Box<Option<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindowDaysOfWeek>>,
    /// Specifies duration of the window in seconds with up to nine fractional digits,
    /// terminated by 's'. Example: "3.5s". Restrictions for duration based on the
    /// recurrence type to allow some time for backup to happen:
    /// - single_occurrence_date:  no restriction
    /// - daily window: duration < 24 hours
    /// - weekly window:
    /// - days of week includes all seven days of a week: duration < 24 hours
    /// - all other weekly window: duration < 168 hours (i.e., 24 * 7 hours)
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// No recurrence. The exclusion window occurs only once and on this date in UTC.
    /// Only one of singleOccurrenceDate, daily and daysOfWeek may be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "singleOccurrenceDate")]
    pub r#single_occurrence_date: Box<Option<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindowSingleOccurrenceDate>>,
    /// Specifies the start time of the window using time of the day in UTC.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime>,
}
