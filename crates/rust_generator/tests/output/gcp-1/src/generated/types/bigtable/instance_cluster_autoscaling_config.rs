#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceClusterAutoscalingConfig {
    /// The target CPU utilization for autoscaling, in percentage. Must be between 10 and 80.
    #[builder(into)]
    #[serde(rename = "cpuTarget")]
    pub r#cpu_target: Box<i32>,
    /// The maximum number of nodes for autoscaling.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Box<i32>,
    /// The minimum number of nodes for autoscaling.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Box<i32>,
    /// The target storage utilization for autoscaling, in GB, for each node in a cluster. This number is limited between 2560 (2.5TiB) and 5120 (5TiB) for a SSD cluster and between 8192 (8TiB) and 16384 (16 TiB) for an HDD cluster. If not set, whatever is already set for the cluster will not change, or if the cluster is just being created, it will use the default value of 2560 for SSD clusters and 8192 for HDD clusters.
    /// 
    /// !> **Warning**: Only one of `autoscaling_config` or `num_nodes` should be set for a cluster. If both are set, `num_nodes` is ignored. If none is set, autoscaling will be disabled and sized to the current node count.
    #[builder(into, default)]
    #[serde(rename = "storageTarget")]
    pub r#storage_target: Box<Option<i32>>,
}
