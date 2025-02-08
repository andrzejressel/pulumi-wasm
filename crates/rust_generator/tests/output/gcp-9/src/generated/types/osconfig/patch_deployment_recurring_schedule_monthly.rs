#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PatchDeploymentRecurringScheduleMonthly {
    /// One day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month.
    /// Months without the target day will be skipped. For example, a schedule to run "every month on the 31st"
    /// will not run in February, April, June, etc.
    #[builder(into, default)]
    #[serde(rename = "monthDay")]
    pub r#month_day: Box<Option<i32>>,
    /// Week day in a month.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "weekDayOfMonth")]
    pub r#week_day_of_month: Box<Option<super::super::types::osconfig::PatchDeploymentRecurringScheduleMonthlyWeekDayOfMonth>>,
}
