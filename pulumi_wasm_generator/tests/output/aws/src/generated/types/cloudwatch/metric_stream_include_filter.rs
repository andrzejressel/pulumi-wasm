#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricStreamIncludeFilter {
    /// An array that defines the metrics you want to include for this metric namespace
    #[builder(into, default)]
    #[serde(rename = "metricNames")]
    pub r#metric_names: Box<Option<Vec<String>>>,
    /// Name of the metric namespace in the filter.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
}
