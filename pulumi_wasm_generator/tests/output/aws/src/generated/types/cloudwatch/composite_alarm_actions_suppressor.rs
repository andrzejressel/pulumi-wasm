#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CompositeAlarmActionsSuppressor {
    /// Can be an AlarmName or an Amazon Resource Name (ARN) from an existing alarm.
    #[builder(into)]
    #[serde(rename = "alarm")]
    pub r#alarm: Box<String>,
    /// The maximum time in seconds that the composite alarm waits after suppressor alarm goes out of the `ALARM` state. After this time, the composite alarm performs its actions.
    #[builder(into)]
    #[serde(rename = "extensionPeriod")]
    pub r#extension_period: Box<i32>,
    /// The maximum time in seconds that the composite alarm waits for the suppressor alarm to go into the `ALARM` state. After this time, the composite alarm performs its actions.
    #[builder(into)]
    #[serde(rename = "waitPeriod")]
    pub r#wait_period: Box<i32>,
}
