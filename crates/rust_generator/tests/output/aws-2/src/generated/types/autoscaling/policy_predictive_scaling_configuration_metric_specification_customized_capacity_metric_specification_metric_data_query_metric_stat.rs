#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQueryMetricStat {
    /// Structure that defines the CloudWatch metric to return, including the metric name, namespace, and dimensions.
    #[builder(into)]
    #[serde(rename = "metric")]
    pub r#metric: Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQueryMetricStatMetric>,
    /// Statistic of the metrics to return.
    #[builder(into)]
    #[serde(rename = "stat")]
    pub r#stat: Box<String>,
    /// Unit of the metrics to return.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
}
