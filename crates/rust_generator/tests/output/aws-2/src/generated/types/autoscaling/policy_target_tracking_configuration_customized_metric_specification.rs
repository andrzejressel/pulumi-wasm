#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyTargetTrackingConfigurationCustomizedMetricSpecification {
    /// Dimensions of the metric.
    #[builder(into, default)]
    #[serde(rename = "metricDimensions")]
    pub r#metric_dimensions: Box<Option<Vec<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricDimension>>>,
    /// Name of the metric.
    #[builder(into, default)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<Option<String>>,
    /// Metrics to include, as a metric data query.
    #[builder(into, default)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Option<Vec<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetric>>>,
    /// Namespace of the metric.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
    /// Statistic of the metric.
    #[builder(into, default)]
    #[serde(rename = "statistic")]
    pub r#statistic: Box<Option<String>>,
    /// Unit of the metric.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
}
