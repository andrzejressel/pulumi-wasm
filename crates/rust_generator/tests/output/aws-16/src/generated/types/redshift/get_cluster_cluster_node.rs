#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterClusterNode {
    /// Whether the node is a leader node or a compute node
    #[builder(into)]
    #[serde(rename = "nodeRole")]
    pub r#node_role: Box<String>,
    /// Private IP address of a node within a cluster
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// Public IP address of a node within a cluster
    #[builder(into)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: Box<String>,
}
