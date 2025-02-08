#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ComputeClusterScaleSettings {
    /// Maximum node count. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<i32>,
    /// Minimal node count. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<i32>,
    /// Node Idle Time Before Scale Down: defines the time until the compute is shutdown when it has gone into Idle state. Is defined according to W3C XML schema standard for duration. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into)]
    #[serde(rename = "scaleDownNodesAfterIdleDuration")]
    pub r#scale_down_nodes_after_idle_duration: Box<String>,
}
