#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduleDailyRecurrence {
    /// The time each day when the schedule takes effect.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<String>,
}