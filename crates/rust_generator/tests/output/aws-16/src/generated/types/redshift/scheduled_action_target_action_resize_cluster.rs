#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduledActionTargetActionResizeCluster {
    /// A boolean value indicating whether the resize operation is using the classic resize process. Default: `false`.
    #[builder(into, default)]
    #[serde(rename = "classic")]
    pub r#classic: Box<Option<bool>>,
    /// The unique identifier for the cluster to resize.
    #[builder(into)]
    #[serde(rename = "clusterIdentifier")]
    pub r#cluster_identifier: Box<String>,
    /// The new cluster type for the specified cluster.
    #[builder(into, default)]
    #[serde(rename = "clusterType")]
    pub r#cluster_type: Box<Option<String>>,
    /// The new node type for the nodes you are adding.
    #[builder(into, default)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Box<Option<String>>,
    /// The new number of nodes for the cluster.
    #[builder(into, default)]
    #[serde(rename = "numberOfNodes")]
    pub r#number_of_nodes: Box<Option<i32>>,
}
