#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterPrivateClusterConfig {
    /// When true, the cluster's private endpoint is used as the cluster endpoint and access through the public endpoint is disabled. When false, either endpoint can be used.
    #[builder(into)]
    #[serde(rename = "enablePrivateEndpoint")]
    pub r#enable_private_endpoint: Box<bool>,
    /// Enables the private cluster feature, creating a private endpoint on the cluster. In a private cluster, nodes only have RFC 1918 private addresses and communicate with the master's private endpoint via private networking.
    #[builder(into)]
    #[serde(rename = "enablePrivateNodes")]
    pub r#enable_private_nodes: Box<bool>,
    /// Controls cluster master global access settings.
    #[builder(into)]
    #[serde(rename = "masterGlobalAccessConfigs")]
    pub r#master_global_access_configs: Box<Vec<super::super::types::container::GetClusterPrivateClusterConfigMasterGlobalAccessConfig>>,
    /// The IP range in CIDR notation to use for the hosted master network. This range will be used for assigning private IP addresses to the cluster master(s) and the ILB VIP. This range must not overlap with any other ranges in use within the cluster's network, and it must be a /28 subnet. See Private Cluster Limitations for more details. This field only applies to private clusters, when enable_private_nodes is true.
    #[builder(into)]
    #[serde(rename = "masterIpv4CidrBlock")]
    pub r#master_ipv_4_cidr_block: Box<String>,
    /// The name of the peering between this cluster and the Google owned VPC.
    #[builder(into)]
    #[serde(rename = "peeringName")]
    pub r#peering_name: Box<String>,
    /// The internal IP address of this cluster's master endpoint.
    #[builder(into)]
    #[serde(rename = "privateEndpoint")]
    pub r#private_endpoint: Box<String>,
    /// Subnetwork in cluster's network where master's endpoint will be provisioned.
    #[builder(into)]
    #[serde(rename = "privateEndpointSubnetwork")]
    pub r#private_endpoint_subnetwork: Box<String>,
    /// The external IP address of this cluster's master endpoint.
    #[builder(into)]
    #[serde(rename = "publicEndpoint")]
    pub r#public_endpoint: Box<String>,
}
