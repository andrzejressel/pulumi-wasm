#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CapacityProviderAutoScalingGroupProvider {
    /// ARN of the associated auto scaling group.
    #[builder(into)]
    #[serde(rename = "autoScalingGroupArn")]
    pub r#auto_scaling_group_arn: Box<String>,
    /// Enables or disables a graceful shutdown of instances without disturbing workloads. Valid values are `ENABLED` and `DISABLED`. The default value is `ENABLED` when a capacity provider is created.
    #[builder(into, default)]
    #[serde(rename = "managedDraining")]
    pub r#managed_draining: Box<Option<String>>,
    /// Configuration block defining the parameters of the auto scaling. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "managedScaling")]
    pub r#managed_scaling: Box<Option<super::super::types::ecs::CapacityProviderAutoScalingGroupProviderManagedScaling>>,
    /// Enables or disables container-aware termination of instances in the auto scaling group when scale-in happens. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "managedTerminationProtection")]
    pub r#managed_termination_protection: Box<Option<String>>,
}
