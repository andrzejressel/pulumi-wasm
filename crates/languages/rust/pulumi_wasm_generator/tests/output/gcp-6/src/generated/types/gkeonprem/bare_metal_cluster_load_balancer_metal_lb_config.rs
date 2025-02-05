#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterLoadBalancerMetalLbConfig {
    /// AddressPools is a list of non-overlapping IP pools used by load balancer
    /// typed services. All addresses must be routable to load balancer nodes.
    /// IngressVIP must be included in the pools.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "addressPools")]
    pub r#address_pools: Box<Vec<super::super::types::gkeonprem::BareMetalClusterLoadBalancerMetalLbConfigAddressPool>>,
    /// Specifies the load balancer's node pool configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerNodePoolConfig")]
    pub r#load_balancer_node_pool_config: Box<Option<super::super::types::gkeonprem::BareMetalClusterLoadBalancerMetalLbConfigLoadBalancerNodePoolConfig>>,
}
