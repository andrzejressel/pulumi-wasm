#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduleHourlyRecurrence {
    /// Minutes of the hour the schedule will run.
    #[builder(into)]
    #[serde(rename = "minute")]
    pub r#minute: Box<i32>,
}