#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetScalingConfigurationTargetTrackingScalingConfig {
    /// Metric type to determine auto-scaling. Valid value: `FLEET_UTILIZATION_RATE`.
    #[builder(into, default)]
    #[serde(rename = "metricType")]
    pub r#metric_type: Box<Option<String>>,
    /// Value of metricType when to start scaling.
    #[builder(into, default)]
    #[serde(rename = "targetValue")]
    pub r#target_value: Box<Option<f64>>,
}
