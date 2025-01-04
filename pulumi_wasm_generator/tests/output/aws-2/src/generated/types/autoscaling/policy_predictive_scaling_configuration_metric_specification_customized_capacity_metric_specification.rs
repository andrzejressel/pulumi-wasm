#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecification {
    /// List of up to 10 structures that defines custom capacity metric in predictive scaling policy
    #[builder(into)]
    #[serde(rename = "metricDataQueries")]
    pub r#metric_data_queries: Box<Vec<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecificationMetricDataQuery>>,
}
