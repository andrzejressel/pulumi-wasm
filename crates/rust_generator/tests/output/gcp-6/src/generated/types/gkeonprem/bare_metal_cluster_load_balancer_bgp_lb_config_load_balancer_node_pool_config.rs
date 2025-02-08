#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfig {
    /// The generic configuration for a node pool running a load balancer.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nodePoolConfig")]
    pub r#node_pool_config: Box<Option<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfigNodePoolConfig>>,
}
