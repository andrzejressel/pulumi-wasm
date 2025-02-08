#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterNetworkProfileLoadBalancerProfile {
    /// The type of the managed inbound Load Balancer Backend Pool. Possible values are `NodeIP` and `NodeIPConfiguration`. Defaults to `NodeIPConfiguration`. See [the documentation](https://learn.microsoft.com/en-us/azure/aks/load-balancer-standard#change-the-inbound-pool-type) for more information.
    #[builder(into, default)]
    #[serde(rename = "backendPoolType")]
    pub r#backend_pool_type: Box<Option<String>>,
    /// The outcome (resource IDs) of the specified arguments.
    #[builder(into, default)]
    #[serde(rename = "effectiveOutboundIps")]
    pub r#effective_outbound_ips: Box<Option<Vec<String>>>,
    /// Desired outbound flow idle timeout in minutes for the cluster load balancer. Must be between `4` and `100` inclusive. Defaults to `30`.
    #[builder(into, default)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Box<Option<i32>>,
    /// Count of desired managed outbound IPs for the cluster load balancer. Must be between `1` and `100` inclusive.
    #[builder(into, default)]
    #[serde(rename = "managedOutboundIpCount")]
    pub r#managed_outbound_ip_count: Box<Option<i32>>,
    /// The desired number of IPv6 outbound IPs created and managed by Azure for the cluster load balancer. Must be in the range of 1 to 100 (inclusive). The default value is 0 for single-stack and 1 for dual-stack.
    /// 
    /// > **Note:** `managed_outbound_ipv6_count` requires dual-stack networking. To enable dual-stack networking the Preview Feature `Microsoft.ContainerService/AKS-EnableDualStack` needs to be enabled and the Resource Provider re-registered, see [the documentation](https://docs.microsoft.com/azure/aks/configure-kubenet-dual-stack?tabs=azure-cli%2Ckubectl#register-the-aks-enabledualstack-preview-feature) for more information.
    #[builder(into, default)]
    #[serde(rename = "managedOutboundIpv6Count")]
    pub r#managed_outbound_ipv_6_count: Box<Option<i32>>,
    /// The ID of the Public IP Addresses which should be used for outbound communication for the cluster load balancer.
    /// 
    /// > **Note:** Set `outbound_ip_address_ids` to an empty slice `[]` in order to unlink it from the cluster. Unlinking a `outbound_ip_address_ids` will revert the load balancing for the cluster back to a managed one.
    #[builder(into, default)]
    #[serde(rename = "outboundIpAddressIds")]
    pub r#outbound_ip_address_ids: Box<Option<Vec<String>>>,
    /// The ID of the outbound Public IP Address Prefixes which should be used for the cluster load balancer.
    /// 
    /// > **Note:** Set `outbound_ip_prefix_ids` to an empty slice `[]` in order to unlink it from the cluster. Unlinking a `outbound_ip_prefix_ids` will revert the load balancing for the cluster back to a managed one.
    #[builder(into, default)]
    #[serde(rename = "outboundIpPrefixIds")]
    pub r#outbound_ip_prefix_ids: Box<Option<Vec<String>>>,
    /// Number of desired SNAT port for each VM in the clusters load balancer. Must be between `0` and `64000` inclusive. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "outboundPortsAllocated")]
    pub r#outbound_ports_allocated: Box<Option<i32>>,
}
