#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScalingPlanScalingInstructionTargetTrackingConfiguration {
    /// Customized metric. You can specify either `customized_scaling_metric_specification` or `predefined_scaling_metric_specification`.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_CustomizedScalingMetricSpecification.html).
    #[builder(into, default)]
    #[serde(rename = "customizedScalingMetricSpecification")]
    pub r#customized_scaling_metric_specification: Box<Option<super::super::types::autoscalingplans::ScalingPlanScalingInstructionTargetTrackingConfigurationCustomizedScalingMetricSpecification>>,
    /// Boolean indicating whether scale in by the target tracking scaling policy is disabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "disableScaleIn")]
    pub r#disable_scale_in: Box<Option<bool>>,
    /// Estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics.
    /// This value is used only if the resource is an Auto Scaling group.
    #[builder(into, default)]
    #[serde(rename = "estimatedInstanceWarmup")]
    pub r#estimated_instance_warmup: Box<Option<i32>>,
    /// Predefined metric. You can specify either `predefined_scaling_metric_specification` or `customized_scaling_metric_specification`.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_PredefinedScalingMetricSpecification.html).
    #[builder(into, default)]
    #[serde(rename = "predefinedScalingMetricSpecification")]
    pub r#predefined_scaling_metric_specification: Box<Option<super::super::types::autoscalingplans::ScalingPlanScalingInstructionTargetTrackingConfigurationPredefinedScalingMetricSpecification>>,
    /// Amount of time, in seconds, after a scale in activity completes before another scale in activity can start.
    /// This value is not used if the scalable resource is an Auto Scaling group.
    #[builder(into, default)]
    #[serde(rename = "scaleInCooldown")]
    pub r#scale_in_cooldown: Box<Option<i32>>,
    /// Amount of time, in seconds, after a scale-out activity completes before another scale-out activity can start.
    /// This value is not used if the scalable resource is an Auto Scaling group.
    #[builder(into, default)]
    #[serde(rename = "scaleOutCooldown")]
    pub r#scale_out_cooldown: Box<Option<i32>>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: Box<f64>,
}
