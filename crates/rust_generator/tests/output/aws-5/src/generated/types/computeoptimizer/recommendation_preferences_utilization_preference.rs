#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecommendationPreferencesUtilizationPreference {
    /// The name of the resource utilization metric name to customize. Valid values: `CpuUtilization`, `MemoryUtilization`.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<String>,
    /// The parameters to set when customizing the resource utilization thresholds.
    #[builder(into, default)]
    #[serde(rename = "metricParameters")]
    pub r#metric_parameters: Box<Option<super::super::types::computeoptimizer::RecommendationPreferencesUtilizationPreferenceMetricParameters>>,
}
