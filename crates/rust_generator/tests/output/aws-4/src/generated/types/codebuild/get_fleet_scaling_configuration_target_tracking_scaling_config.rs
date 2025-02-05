#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFleetScalingConfigurationTargetTrackingScalingConfig {
    /// The metric type to determine auto-scaling.
    #[builder(into)]
    #[serde(rename = "metricType")]
    pub r#metric_type: Box<String>,
    /// The value of metric_type when to start scaling.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: Box<f64>,
}
