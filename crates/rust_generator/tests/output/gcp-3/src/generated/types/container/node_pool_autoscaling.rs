#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodePoolAutoscaling {
    /// Location policy specifies the algorithm used when
    /// scaling-up the node pool. Location policy is supported only in 1.24.1+ clusters.
    /// * "BALANCED" - Is a best effort policy that aims to balance the sizes of available zones.
    /// * "ANY" - Instructs the cluster autoscaler to prioritize utilization of unused reservations,
    /// and reduce preemption risk for Spot VMs.
    #[builder(into, default)]
    #[serde(rename = "locationPolicy")]
    pub r#location_policy: Box<Option<String>>,
    /// Maximum number of nodes per zone in the NodePool.
    /// Must be >= min_node_count. Cannot be used with total limits.
    #[builder(into, default)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<Option<i32>>,
    /// Minimum number of nodes per zone in the NodePool.
    /// Must be >=0 and <= `max_node_count`. Cannot be used with total limits.
    #[builder(into, default)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<Option<i32>>,
    /// Total maximum number of nodes in the NodePool.
    /// Must be >= total_min_node_count. Cannot be used with per zone limits.
    /// Total size limits are supported only in 1.24.1+ clusters.
    #[builder(into, default)]
    #[serde(rename = "totalMaxNodeCount")]
    pub r#total_max_node_count: Box<Option<i32>>,
    /// Total minimum number of nodes in the NodePool.
    /// Must be >=0 and <= `total_max_node_count`. Cannot be used with per zone limits.
    /// Total size limits are supported only in 1.24.1+ clusters.
    #[builder(into, default)]
    #[serde(rename = "totalMinNodeCount")]
    pub r#total_min_node_count: Box<Option<i32>>,
}
