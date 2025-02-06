#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterPrivateClusterConfig {
    /// When `true`, the cluster's private
    /// endpoint is used as the cluster endpoint and access through the public endpoint
    /// is disabled. When `false`, either endpoint can be used. This field only applies
    /// to private clusters, when `enable_private_nodes` is `true`.
    #[builder(into, default)]
    #[serde(rename = "enablePrivateEndpoint")]
    pub r#enable_private_endpoint: Box<Option<bool>>,
    /// Enables the private cluster feature,
    /// creating a private endpoint on the cluster. In a private cluster, nodes only
    /// have RFC 1918 private addresses and communicate with the master's private
    /// endpoint via private networking.
    #[builder(into, default)]
    #[serde(rename = "enablePrivateNodes")]
    pub r#enable_private_nodes: Box<Option<bool>>,
    /// Controls cluster master global
    /// access settings. If unset, the provider will no longer manage this field and will
    /// not modify the previously-set value. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "masterGlobalAccessConfig")]
    pub r#master_global_access_config: Box<Option<super::super::types::container::ClusterPrivateClusterConfigMasterGlobalAccessConfig>>,
    /// The IP range in CIDR notation to use for
    /// the hosted master network. This range will be used for assigning private IP
    /// addresses to the cluster master(s) and the ILB VIP. This range must not overlap
    /// with any other ranges in use within the cluster's network, and it must be a /28
    /// subnet. See [Private Cluster Limitations](https://cloud.google.com/kubernetes-engine/docs/how-to/private-clusters#req_res_lim)
    /// for more details. This field only applies to private clusters, when
    /// `enable_private_nodes` is `true`.
    #[builder(into, default)]
    #[serde(rename = "masterIpv4CidrBlock")]
    pub r#master_ipv_4_cidr_block: Box<Option<String>>,
    /// The name of the peering between this cluster and the Google owned VPC.
    #[builder(into, default)]
    #[serde(rename = "peeringName")]
    pub r#peering_name: Box<Option<String>>,
    /// The internal IP address of this cluster's master endpoint.
    #[builder(into, default)]
    #[serde(rename = "privateEndpoint")]
    pub r#private_endpoint: Box<Option<String>>,
    /// Subnetwork in cluster's network where master's endpoint will be provisioned.
    #[builder(into, default)]
    #[serde(rename = "privateEndpointSubnetwork")]
    pub r#private_endpoint_subnetwork: Box<Option<String>>,
    /// The external IP address of this cluster's master endpoint.
    /// 
    /// !> The Google provider is unable to validate certain configurations of
    /// `private_cluster_config` when `enable_private_nodes` is `false`. It's
    /// recommended that you omit the block entirely if the field is not set to `true`.
    #[builder(into, default)]
    #[serde(rename = "publicEndpoint")]
    pub r#public_endpoint: Box<Option<String>>,
}
