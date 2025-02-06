#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduleWeeklyRecurrence {
    /// The time when the schedule takes effect.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<String>,
    /// A list of days that this schedule takes effect . Possible values include `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
    #[builder(into, default)]
    #[serde(rename = "weekDays")]
    pub r#week_days: Box<Option<Vec<String>>>,
}
