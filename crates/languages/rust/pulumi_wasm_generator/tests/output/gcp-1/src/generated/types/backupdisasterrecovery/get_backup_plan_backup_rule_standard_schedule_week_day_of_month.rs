#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackupPlanBackupRuleStandardScheduleWeekDayOfMonth {
    /// Specifies the day of the week. Possible values: ["DAY_OF_WEEK_UNSPECIFIED", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY", "SUNDAY"]
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// WeekOfMonth enumerates possible weeks in the month, e.g. the first, third, or last week of the month. Possible values: ["WEEK_OF_MONTH_UNSPECIFIED", "FIRST", "SECOND", "THIRD", "FOURTH", "LAST"]
    #[builder(into)]
    #[serde(rename = "weekOfMonth")]
    pub r#week_of_month: Box<String>,
}
