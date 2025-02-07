#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertProcessingRuleSuppressionSchedule {
    /// Specifies the Alert Processing Rule effective start time (Y-m-d'T'H:M:S).
    #[builder(into, default)]
    #[serde(rename = "effectiveFrom")]
    pub r#effective_from: Box<Option<String>>,
    /// Specifies the Alert Processing Rule effective end time (Y-m-d'T'H:M:S).
    #[builder(into, default)]
    #[serde(rename = "effectiveUntil")]
    pub r#effective_until: Box<Option<String>>,
    /// A `recurrence` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Box<Option<super::super::types::monitoring::AlertProcessingRuleSuppressionScheduleRecurrence>>,
    /// The time zone (e.g. Pacific Standard time, Eastern Standard Time). Defaults to `UTC`. [possible values are defined here](https://docs.microsoft.com/en-us/previous-versions/windows/embedded/ms912391(v=winembedded.11)).
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
}
