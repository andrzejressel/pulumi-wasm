#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VMwareClusterLoadBalancerMetalLbConfig {
    /// AddressPools is a list of non-overlapping IP pools used by load balancer
    /// typed services. All addresses must be routable to load balancer nodes.
    /// IngressVIP must be included in the pools.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "addressPools")]
    pub r#address_pools: Box<Vec<super::super::types::gkeonprem::VMwareClusterLoadBalancerMetalLbConfigAddressPool>>,
}
