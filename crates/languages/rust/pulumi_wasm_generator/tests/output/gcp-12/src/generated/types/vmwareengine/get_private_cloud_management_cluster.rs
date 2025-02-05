#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPrivateCloudManagementCluster {
    /// Configuration of the autoscaling applied to this cluster
    /// Private cloud must have a minimum of 3 nodes to add autoscale settings
    #[builder(into)]
    #[serde(rename = "autoscalingSettings")]
    pub r#autoscaling_settings: Box<Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSetting>>,
    /// The user-provided identifier of the new Cluster. The identifier must meet the following requirements:
    ///   * Only contains 1-63 alphanumeric characters and hyphens
    ///   * Begins with an alphabetical character
    ///   * Ends with a non-hyphen character
    ///   * Not formatted as a UUID
    ///   * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: Box<String>,
    /// The map of cluster node types in this cluster,
    /// where the key is canonical identifier of the node type (corresponds to the NodeType).
    #[builder(into)]
    #[serde(rename = "nodeTypeConfigs")]
    pub r#node_type_configs: Box<Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterNodeTypeConfig>>,
    /// The stretched cluster configuration for the private cloud.
    #[builder(into)]
    #[serde(rename = "stretchedClusterConfigs")]
    pub r#stretched_cluster_configs: Box<Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterStretchedClusterConfig>>,
}
