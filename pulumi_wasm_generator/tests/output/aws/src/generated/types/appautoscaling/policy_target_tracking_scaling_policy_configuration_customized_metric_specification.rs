#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecification {
    /// Dimensions of the metric.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<Vec<super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationDimension>>>,
    /// Name of the metric.
    #[builder(into, default)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<Option<String>>,
    /// Metrics to include, as a metric data query.
    #[builder(into, default)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Option<Vec<super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecificationMetric>>>,
    /// Namespace of the metric.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
    /// Statistic of the metric. Valid values: `Average`, `Minimum`, `Maximum`, `SampleCount`, and `Sum`.
    #[builder(into, default)]
    #[serde(rename = "statistic")]
    pub r#statistic: Box<Option<String>>,
    /// Unit of the metrics to return.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
}