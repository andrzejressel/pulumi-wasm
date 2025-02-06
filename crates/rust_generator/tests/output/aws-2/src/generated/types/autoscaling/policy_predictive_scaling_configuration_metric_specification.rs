#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecification {
    /// Customized capacity metric specification. The field is only valid when you use `customized_load_metric_specification`
    #[builder(into, default)]
    #[serde(rename = "customizedCapacityMetricSpecification")]
    pub r#customized_capacity_metric_specification: Box<Option<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedCapacityMetricSpecification>>,
    /// Customized load metric specification.
    #[builder(into, default)]
    #[serde(rename = "customizedLoadMetricSpecification")]
    pub r#customized_load_metric_specification: Box<Option<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecification>>,
    /// Customized scaling metric specification.
    #[builder(into, default)]
    #[serde(rename = "customizedScalingMetricSpecification")]
    pub r#customized_scaling_metric_specification: Box<Option<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedScalingMetricSpecification>>,
    /// Predefined load metric specification.
    #[builder(into, default)]
    #[serde(rename = "predefinedLoadMetricSpecification")]
    pub r#predefined_load_metric_specification: Box<Option<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedLoadMetricSpecification>>,
    /// Metric pair specification from which Amazon EC2 Auto Scaling determines the appropriate scaling metric and load metric to use.
    #[builder(into, default)]
    #[serde(rename = "predefinedMetricPairSpecification")]
    pub r#predefined_metric_pair_specification: Box<Option<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedMetricPairSpecification>>,
    /// Predefined scaling metric specification.
    #[builder(into, default)]
    #[serde(rename = "predefinedScalingMetricSpecification")]
    pub r#predefined_scaling_metric_specification: Box<Option<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationPredefinedScalingMetricSpecification>>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: Box<f64>,
}
