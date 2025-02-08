#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScalingPlanScalingInstruction {
    /// Customized load metric to use for predictive scaling. You must specify either `customized_load_metric_specification` or `predefined_load_metric_specification` when configuring predictive scaling.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_CustomizedLoadMetricSpecification.html).
    #[builder(into, default)]
    #[serde(rename = "customizedLoadMetricSpecification")]
    pub r#customized_load_metric_specification: Box<Option<super::super::types::autoscalingplans::ScalingPlanScalingInstructionCustomizedLoadMetricSpecification>>,
    /// Boolean controlling whether dynamic scaling by AWS Auto Scaling is disabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "disableDynamicScaling")]
    pub r#disable_dynamic_scaling: Box<Option<bool>>,
    /// Maximum capacity of the resource. The exception to this upper limit is if you specify a non-default setting for `predictive_scaling_max_capacity_behavior`.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<i32>,
    /// Minimum capacity of the resource.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<i32>,
    /// Predefined load metric to use for predictive scaling. You must specify either `predefined_load_metric_specification` or `customized_load_metric_specification` when configuring predictive scaling.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_PredefinedLoadMetricSpecification.html).
    #[builder(into, default)]
    #[serde(rename = "predefinedLoadMetricSpecification")]
    pub r#predefined_load_metric_specification: Box<Option<super::super::types::autoscalingplans::ScalingPlanScalingInstructionPredefinedLoadMetricSpecification>>,
    /// Defines the behavior that should be applied if the forecast capacity approaches or exceeds the maximum capacity specified for the resource.
    /// Valid values: `SetForecastCapacityToMaxCapacity`, `SetMaxCapacityAboveForecastCapacity`, `SetMaxCapacityToForecastCapacity`.
    #[builder(into, default)]
    #[serde(rename = "predictiveScalingMaxCapacityBehavior")]
    pub r#predictive_scaling_max_capacity_behavior: Box<Option<String>>,
    /// Size of the capacity buffer to use when the forecast capacity is close to or exceeds the maximum capacity.
    #[builder(into, default)]
    #[serde(rename = "predictiveScalingMaxCapacityBuffer")]
    pub r#predictive_scaling_max_capacity_buffer: Box<Option<i32>>,
    /// Predictive scaling mode. Valid values: `ForecastAndScale`, `ForecastOnly`.
    #[builder(into, default)]
    #[serde(rename = "predictiveScalingMode")]
    pub r#predictive_scaling_mode: Box<Option<String>>,
    /// ID of the resource. This string consists of the resource type and unique identifier.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
    /// Scalable dimension associated with the resource. Valid values: `autoscaling:autoScalingGroup:DesiredCapacity`, `dynamodb:index:ReadCapacityUnits`, `dynamodb:index:WriteCapacityUnits`, `dynamodb:table:ReadCapacityUnits`, `dynamodb:table:WriteCapacityUnits`, `ecs:service:DesiredCount`, `ec2:spot-fleet-request:TargetCapacity`, `rds:cluster:ReadReplicaCount`.
    #[builder(into)]
    #[serde(rename = "scalableDimension")]
    pub r#scalable_dimension: Box<String>,
    /// Controls whether a resource's externally created scaling policies are kept or replaced. Valid values: `KeepExternalPolicies`, `ReplaceExternalPolicies`. Defaults to `KeepExternalPolicies`.
    #[builder(into, default)]
    #[serde(rename = "scalingPolicyUpdateBehavior")]
    pub r#scaling_policy_update_behavior: Box<Option<String>>,
    /// Amount of time, in seconds, to buffer the run time of scheduled scaling actions when scaling out.
    #[builder(into, default)]
    #[serde(rename = "scheduledActionBufferTime")]
    pub r#scheduled_action_buffer_time: Box<Option<i32>>,
    /// Namespace of the AWS service. Valid values: `autoscaling`, `dynamodb`, `ecs`, `ec2`, `rds`.
    #[builder(into)]
    #[serde(rename = "serviceNamespace")]
    pub r#service_namespace: Box<String>,
    /// Structure that defines new target tracking configurations. Each of these structures includes a specific scaling metric and a target value for the metric, along with various parameters to use with dynamic scaling.
    /// More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_TargetTrackingConfiguration.html).
    #[builder(into)]
    #[serde(rename = "targetTrackingConfigurations")]
    pub r#target_tracking_configurations: Box<Vec<super::super::types::autoscalingplans::ScalingPlanScalingInstructionTargetTrackingConfiguration>>,
}
