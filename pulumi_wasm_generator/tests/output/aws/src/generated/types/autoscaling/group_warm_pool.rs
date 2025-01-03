#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupWarmPool {
    /// Whether instances in the Auto Scaling group can be returned to the warm pool on scale in. The default is to terminate instances in the Auto Scaling group when the group scales in.
    #[builder(into, default)]
    #[serde(rename = "instanceReusePolicy")]
    pub r#instance_reuse_policy: Box<Option<super::super::types::autoscaling::GroupWarmPoolInstanceReusePolicy>>,
    /// Total maximum number of instances that are allowed to be in the warm pool or in any state except Terminated for the Auto Scaling group.
    #[builder(into, default)]
    #[serde(rename = "maxGroupPreparedCapacity")]
    pub r#max_group_prepared_capacity: Box<Option<i32>>,
    /// Minimum number of instances to maintain in the warm pool. This helps you to ensure that there is always a certain number of warmed instances available to handle traffic spikes. Defaults to 0 if not specified.
    #[builder(into, default)]
    #[serde(rename = "minSize")]
    pub r#min_size: Box<Option<i32>>,
    /// Sets the instance state to transition to after the lifecycle hooks finish. Valid values are: Stopped (default), Running or Hibernated.
    #[builder(into, default)]
    #[serde(rename = "poolState")]
    pub r#pool_state: Box<Option<String>>,
}
