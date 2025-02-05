#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceQuotaUsageMetric {
    /// The metric dimensions.
    #[builder(into, default)]
    #[serde(rename = "metricDimensions")]
    pub r#metric_dimensions: Box<Option<Vec<super::super::types::servicequotas::ServiceQuotaUsageMetricMetricDimension>>>,
    /// The name of the metric.
    #[builder(into, default)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<Option<String>>,
    /// The namespace of the metric.
    #[builder(into, default)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: Box<Option<String>>,
    /// The metric statistic that AWS recommend you use when determining quota usage.
    #[builder(into, default)]
    #[serde(rename = "metricStatisticRecommendation")]
    pub r#metric_statistic_recommendation: Box<Option<String>>,
}
