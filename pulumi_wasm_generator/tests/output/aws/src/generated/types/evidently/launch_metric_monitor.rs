#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchMetricMonitor {
    /// A block that defines the metric. Detailed below.
    #[builder(into)]
    #[serde(rename = "metricDefinition")]
    pub r#metric_definition: Box<super::super::types::evidently::LaunchMetricMonitorMetricDefinition>,
}
