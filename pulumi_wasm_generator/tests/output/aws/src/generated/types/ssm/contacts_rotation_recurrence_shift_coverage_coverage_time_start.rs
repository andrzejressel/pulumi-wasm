#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactsRotationRecurrenceShiftCoverageCoverageTimeStart {
    /// (Required) The hour of the day.
    #[builder(into)]
    #[serde(rename = "hourOfDay")]
    pub r#hour_of_day: Box<i32>,
    /// (Required) The minutes of the hour.
    #[builder(into)]
    #[serde(rename = "minuteOfHour")]
    pub r#minute_of_hour: Box<i32>,
}