#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteriaRecurrence {
    /// The end time of the schedule, formatted as an RFC3339 string.
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// The frequency of the schedule. Possible values are `Day`, `Hour`, `Minute`, `Month`, `NotSpecified`, `Second`, `Week` and `Year`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<String>,
    /// The number of `frequency`s between runs.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    /// A `schedule` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteriaRecurrenceSchedule>>,
    /// The start time of the schedule, formatted as an RFC3339 string.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
    /// The timezone of the start/end time.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
}
