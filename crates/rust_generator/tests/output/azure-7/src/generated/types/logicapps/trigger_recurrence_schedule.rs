#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerRecurrenceSchedule {
    /// Specifies a list of hours when the trigger should run. Valid values are between 0 and 23.
    #[builder(into, default)]
    #[serde(rename = "atTheseHours")]
    pub r#at_these_hours: Box<Option<Vec<i32>>>,
    /// Specifies a list of minutes when the trigger should run. Valid values are between 0 and 59.
    #[builder(into, default)]
    #[serde(rename = "atTheseMinutes")]
    pub r#at_these_minutes: Box<Option<Vec<i32>>>,
    /// Specifies a list of days when the trigger should run. Valid values include `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`.
    #[builder(into, default)]
    #[serde(rename = "onTheseDays")]
    pub r#on_these_days: Box<Option<Vec<String>>>,
}
