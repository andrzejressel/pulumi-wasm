#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyTargetTrackingScalingPolicyConfiguration {
    /// Custom CloudWatch metric. Documentation can be found  at: [AWS Customized Metric Specification](https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_CustomizedMetricSpecification.html). See supported fields below.
    #[builder(into, default)]
    #[serde(rename = "customizedMetricSpecification")]
    pub r#customized_metric_specification: Box<Option<super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfigurationCustomizedMetricSpecification>>,
    /// Whether scale in by the target tracking policy is disabled. If the value is true, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is `false`.
    #[builder(into, default)]
    #[serde(rename = "disableScaleIn")]
    pub r#disable_scale_in: Box<Option<bool>>,
    /// Predefined metric. See supported fields below.
    #[builder(into, default)]
    #[serde(rename = "predefinedMetricSpecification")]
    pub r#predefined_metric_specification: Box<Option<super::super::types::appautoscaling::PolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification>>,
    /// Amount of time, in seconds, after a scale in activity completes before another scale in activity can start.
    #[builder(into, default)]
    #[serde(rename = "scaleInCooldown")]
    pub r#scale_in_cooldown: Box<Option<i32>>,
    /// Amount of time, in seconds, after a scale out activity completes before another scale out activity can start.
    #[builder(into, default)]
    #[serde(rename = "scaleOutCooldown")]
    pub r#scale_out_cooldown: Box<Option<i32>>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: Box<f64>,
}
