#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSourceMappingMetricsConfig {
    /// A list containing the metrics to be produced by the event source mapping. Valid values: `EventCount`.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Vec<String>>,
}
