#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CapacityProviderAutoScalingGroupProviderManagedScaling {
    /// Period of time, in seconds, after a newly launched Amazon EC2 instance can contribute to CloudWatch metrics for Auto Scaling group. If this parameter is omitted, the default value of 300 seconds is used.
    /// 
    /// For more information on how the instance warmup period contributes to managed scale-out behavior, see [Control the instances Amazon ECS terminates](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/managed-termination-protection.html) in the _Amazon Elastic Container Service Developer Guide_.
    #[builder(into, default)]
    #[serde(rename = "instanceWarmupPeriod")]
    pub r#instance_warmup_period: Box<Option<i32>>,
    /// Maximum step adjustment size. A number between 1 and 10,000.
    #[builder(into, default)]
    #[serde(rename = "maximumScalingStepSize")]
    pub r#maximum_scaling_step_size: Box<Option<i32>>,
    /// Minimum step adjustment size. A number between 1 and 10,000.
    #[builder(into, default)]
    #[serde(rename = "minimumScalingStepSize")]
    pub r#minimum_scaling_step_size: Box<Option<i32>>,
    /// Whether auto scaling is managed by ECS. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Target utilization for the capacity provider. A number between 1 and 100.
    #[builder(into, default)]
    #[serde(rename = "targetCapacity")]
    pub r#target_capacity: Box<Option<i32>>,
}
