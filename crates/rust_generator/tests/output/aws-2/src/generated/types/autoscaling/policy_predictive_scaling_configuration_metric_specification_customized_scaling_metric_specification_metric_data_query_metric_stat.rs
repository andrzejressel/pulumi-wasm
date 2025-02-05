#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQueryMetricStat {
    /// Structure that defines the CloudWatch metric to return, including the metric name, namespace, and dimensions.
    #[builder(into)]
    #[serde(rename = "metric")]
    pub r#metric: Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecificationMetricDataQueryMetricStatMetric>,
    /// Statistic of the metrics to return.
    #[builder(into)]
    #[serde(rename = "stat")]
    pub r#stat: Box<String>,
    /// Unit of the metrics to return.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
}
