#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MemcachedLayerLoadBasedAutoScalingDownscaling {
    #[builder(into, default)]
    #[serde(rename = "alarms")]
    pub r#alarms: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "cpuThreshold")]
    pub r#cpu_threshold: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "ignoreMetricsTime")]
    pub r#ignore_metrics_time: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "loadThreshold")]
    pub r#load_threshold: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "memoryThreshold")]
    pub r#memory_threshold: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "thresholdsWaitTime")]
    pub r#thresholds_wait_time: Box<Option<i32>>,
}
