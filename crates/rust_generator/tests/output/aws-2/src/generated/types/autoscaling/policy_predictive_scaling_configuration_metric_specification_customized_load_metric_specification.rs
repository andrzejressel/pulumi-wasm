#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecification {
    /// List of up to 10 structures that defines custom load metric in predictive scaling policy
    #[builder(into)]
    #[serde(rename = "metricDataQueries")]
    pub r#metric_data_queries: Box<Vec<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecificationMetricDataQuery>>,
}
