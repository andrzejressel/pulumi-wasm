#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQuery {
    /// Math expression used on the returned metric. You must specify either `expression` or `metric_stat`, but not both.
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Short name for the metric used in predictive scaling policy.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Human-readable label for this metric or expression.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// Structure that defines CloudWatch metric to be used in predictive scaling policy. You must specify either `expression` or `metric_stat`, but not both.
    #[builder(into, default)]
    #[serde(rename = "metricStat")]
    pub r#metric_stat: Box<Option<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQueryMetricStat>>,
    /// Boolean that indicates whether to return the timestamps and raw data values of this metric, the default is true
    #[builder(into, default)]
    #[serde(rename = "returnData")]
    pub r#return_data: Box<Option<bool>>,
}
