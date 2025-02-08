#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertProcessingRuleSuppressionScheduleRecurrenceWeekly {
    /// Specifies a list of dayOfWeek to recurrence. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, and `Saturday`.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Vec<String>>,
    /// Specifies the recurrence end time (H:M:S).
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// Specifies the recurrence start time (H:M:S).
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}
