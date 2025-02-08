#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackupPlanBackupRuleStandardSchedule {
    /// A BackupWindow defines the window of the day during which backup jobs will run. Jobs are queued at the beginning of the window and will be marked as
    /// 'NOT_RUN' if they do not start by the end of the window.
    #[builder(into)]
    #[serde(rename = "backupWindows")]
    pub r#backup_windows: Box<Vec<super::super::types::backupdisasterrecovery::GetBackupPlanBackupRuleStandardScheduleBackupWindow>>,
    /// Specifies days of months like 1, 5, or 14 on which jobs will run.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Box<Vec<i32>>,
    /// Specifies days of week like MONDAY or TUESDAY, on which jobs will run. This is required for 'recurrence_type', 'WEEKLY' and is not applicable otherwise. Possible values: ["DAY_OF_WEEK_UNSPECIFIED", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"]
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Vec<String>>,
    /// Specifies frequency for hourly backups. An hourly frequency of 2 means jobs will run every 2 hours from start time till end time defined.
    /// This is required for 'recurrence_type', 'HOURLY' and is not applicable otherwise.
    #[builder(into)]
    #[serde(rename = "hourlyFrequency")]
    pub r#hourly_frequency: Box<i32>,
    /// Specifies values of months Possible values: ["MONTH_UNSPECIFIED", "JANUARY", "FEBRUARY", "MARCH", "APRIL", "MAY", "JUNE", "JULY", "AUGUST", "SEPTEMBER", "OCTOBER", "NOVEMBER", "DECEMBER"]
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Box<Vec<String>>,
    /// RecurrenceType enumerates the applicable periodicity for the schedule. Possible values: ["HOURLY", "DAILY", "WEEKLY", "MONTHLY", "YEARLY"]
    #[builder(into)]
    #[serde(rename = "recurrenceType")]
    pub r#recurrence_type: Box<String>,
    /// The time zone to be used when interpreting the schedule.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
    /// Specifies a week day of the month like FIRST SUNDAY or LAST MONDAY, on which jobs will run.
    #[builder(into)]
    #[serde(rename = "weekDayOfMonths")]
    pub r#week_day_of_months: Box<Vec<super::super::types::backupdisasterrecovery::GetBackupPlanBackupRuleStandardScheduleWeekDayOfMonth>>,
}
