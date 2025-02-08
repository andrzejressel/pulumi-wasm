#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PrivateCloudManagementCluster {
    /// Configuration of the autoscaling applied to this cluster
    /// Private cloud must have a minimum of 3 nodes to add autoscale settings
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "autoscalingSettings")]
    pub r#autoscaling_settings: Box<Option<super::super::types::vmwareengine::PrivateCloudManagementClusterAutoscalingSettings>>,
    /// The user-provided identifier of the new Cluster. The identifier must meet the following requirements:
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: Box<String>,
    /// The map of cluster node types in this cluster,
    /// where the key is canonical identifier of the node type (corresponds to the NodeType).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nodeTypeConfigs")]
    pub r#node_type_configs: Box<Option<Vec<super::super::types::vmwareengine::PrivateCloudManagementClusterNodeTypeConfig>>>,
    /// The stretched cluster configuration for the private cloud.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "stretchedClusterConfig")]
    pub r#stretched_cluster_config: Box<Option<super::super::types::vmwareengine::PrivateCloudManagementClusterStretchedClusterConfig>>,
}
