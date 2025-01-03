#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceMaintenanceSchedule {
    /// (Output)
    /// Output only. The end time of any upcoming scheduled maintenance for this instance.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// (Output)
    /// Output only. The deadline that the maintenance schedule start time
    /// can not go beyond, including reschedule.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into, default)]
    #[serde(rename = "scheduleDeadlineTime")]
    pub r#schedule_deadline_time: Box<Option<String>>,
    /// (Output)
    /// Output only. The start time of any upcoming scheduled maintenance for this instance.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}
