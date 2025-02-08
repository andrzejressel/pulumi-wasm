#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigAutoscaling {
    /// The maximum number of nodes in the node pool. Must be >= minNodeCount, and must be > 0.
    #[builder(into, default)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<Option<i32>>,
    /// The minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount.
    #[builder(into, default)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<Option<i32>>,
}
