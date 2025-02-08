#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterNodePoolAutoscaling {
    /// Location policy specifies the algorithm used when scaling-up the node pool. "BALANCED" - Is a best effort policy that aims to balance the sizes of available zones. "ANY" - Instructs the cluster autoscaler to prioritize utilization of unused reservations, and reduces preemption risk for Spot VMs.
    #[builder(into, default)]
    #[serde(rename = "locationPolicy")]
    pub r#location_policy: Box<Option<String>>,
    /// Maximum number of nodes per zone in the node pool. Must be >= min_node_count. Cannot be used with total limits.
    #[builder(into, default)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<Option<i32>>,
    /// Minimum number of nodes per zone in the node pool. Must be >=0 and <= max_node_count. Cannot be used with total limits.
    #[builder(into, default)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<Option<i32>>,
    /// Maximum number of all nodes in the node pool. Must be >= total_min_node_count. Cannot be used with per zone limits.
    #[builder(into, default)]
    #[serde(rename = "totalMaxNodeCount")]
    pub r#total_max_node_count: Box<Option<i32>>,
    /// Minimum number of all nodes in the node pool. Must be >=0 and <= total_max_node_count. Cannot be used with per zone limits.
    #[builder(into, default)]
    #[serde(rename = "totalMinNodeCount")]
    pub r#total_min_node_count: Box<Option<i32>>,
}
