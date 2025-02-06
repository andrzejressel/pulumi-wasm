#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterNode {
    /// Whether the node is a leader node or a compute node
    #[builder(into, default)]
    #[serde(rename = "nodeRole")]
    pub r#node_role: Box<Option<String>>,
    /// The private IP address of a node within a cluster
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The public IP address of a node within a cluster
    #[builder(into, default)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: Box<Option<String>>,
}
