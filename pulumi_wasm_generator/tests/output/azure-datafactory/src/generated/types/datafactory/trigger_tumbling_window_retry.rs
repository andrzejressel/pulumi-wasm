#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerTumblingWindowRetry {
    /// The maximum retry attempts if the pipeline run failed.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The Interval in seconds between each retry if the pipeline run failed. Defaults to `30`.
    #[builder(into, default)]
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<i32>>,
}