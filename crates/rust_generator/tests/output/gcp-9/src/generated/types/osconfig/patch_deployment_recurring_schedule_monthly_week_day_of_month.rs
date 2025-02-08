#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PatchDeploymentRecurringScheduleMonthlyWeekDayOfMonth {
    /// A day of the week.
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// Represents the number of days before or after the given week day of month that the patch deployment is scheduled for.
    #[builder(into, default)]
    #[serde(rename = "dayOffset")]
    pub r#day_offset: Box<Option<i32>>,
    /// Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month.
    #[builder(into)]
    #[serde(rename = "weekOrdinal")]
    pub r#week_ordinal: Box<i32>,
}
