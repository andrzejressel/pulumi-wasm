#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScalingPlanScalingInstructionTargetTrackingConfigurationCustomizedScalingMetricSpecification {
    /// Dimensions of the metric.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<std::collections::HashMap<String, String>>>,
    /// Name of the metric.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<String>,
    /// Namespace of the metric.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
    /// Statistic of the metric. Valid values: `Average`, `Maximum`, `Minimum`, `SampleCount`, `Sum`.
    #[builder(into)]
    #[serde(rename = "statistic")]
    pub r#statistic: Box<String>,
    /// Unit of the metric.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
}
