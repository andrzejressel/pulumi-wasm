#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterLoadBalancerBgpLbConfig {
    /// AddressPools is a list of non-overlapping IP pools used by load balancer
    /// typed services. All addresses must be routable to load balancer nodes.
    /// IngressVIP must be included in the pools.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "addressPools")]
    pub r#address_pools: Box<Vec<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfigAddressPool>>,
    /// BGP autonomous system number (ASN) of the cluster.
    /// This field can be updated after cluster creation.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Box<i32>,
    /// The list of BGP peers that the cluster will connect to.
    /// At least one peer must be configured for each control plane node.
    /// Control plane nodes will connect to these peers to advertise the control
    /// plane VIP. The Services load balancer also uses these peers by default.
    /// This field can be updated after cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bgpPeerConfigs")]
    pub r#bgp_peer_configs: Box<Vec<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfigBgpPeerConfig>>,
    /// Specifies the node pool running data plane load balancing. L2 connectivity
    /// is required among nodes in this pool. If missing, the control plane node
    /// pool is used for data plane load balancing.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerNodePoolConfig")]
    pub r#load_balancer_node_pool_config: Box<Option<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfig>>,
}
