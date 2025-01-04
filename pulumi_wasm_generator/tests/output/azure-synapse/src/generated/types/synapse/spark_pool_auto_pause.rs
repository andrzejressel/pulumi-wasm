#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SparkPoolAutoPause {
    /// Number of minutes of idle time before the Spark Pool is automatically paused. Must be between `5` and `10080`.
    #[builder(into)]
    #[serde(rename = "delayInMinutes")]
    pub r#delay_in_minutes: Box<i32>,
}
