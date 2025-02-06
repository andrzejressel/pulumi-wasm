#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertProcessingRuleSuppressionScheduleRecurrenceMonthly {
    /// Specifies a list of dayOfMonth to recurrence. Possible values are integers between `1` - `31`.
    #[builder(into)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Box<Vec<i32>>,
    /// Specifies the recurrence end time (H:M:S).
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// Specifies the recurrence start time (H:M:S).
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}
