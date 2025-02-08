#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpnGatewayConnectionVpnLink {
    /// The expected connection bandwidth in MBPS. Defaults to `10`.
    #[builder(into, default)]
    #[serde(rename = "bandwidthMbps")]
    pub r#bandwidth_mbps: Box<Option<i32>>,
    /// Should the BGP be enabled? Defaults to `false`. Changing this forces a new VPN Gateway Connection to be created.
    #[builder(into, default)]
    #[serde(rename = "bgpEnabled")]
    pub r#bgp_enabled: Box<Option<bool>>,
    /// The connection mode of this VPN Link. Possible values are `Default`, `InitiatorOnly` and `ResponderOnly`. Defaults to `Default`.
    #[builder(into, default)]
    #[serde(rename = "connectionMode")]
    pub r#connection_mode: Box<Option<String>>,
    /// One or more `custom_bgp_address` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "customBgpAddresses")]
    pub r#custom_bgp_addresses: Box<Option<Vec<super::super::types::network::VpnGatewayConnectionVpnLinkCustomBgpAddress>>>,
    /// A list of the egress NAT Rule Ids.
    #[builder(into, default)]
    #[serde(rename = "egressNatRuleIds")]
    pub r#egress_nat_rule_ids: Box<Option<Vec<String>>>,
    /// A list of the ingress NAT Rule Ids.
    #[builder(into, default)]
    #[serde(rename = "ingressNatRuleIds")]
    pub r#ingress_nat_rule_ids: Box<Option<Vec<String>>>,
    /// One or more `ipsec_policy` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "ipsecPolicies")]
    pub r#ipsec_policies: Box<Option<Vec<super::super::types::network::VpnGatewayConnectionVpnLinkIpsecPolicy>>>,
    /// Whether to use local Azure IP to initiate connection? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "localAzureIpAddressEnabled")]
    pub r#local_azure_ip_address_enabled: Box<Option<bool>>,
    /// The name which should be used for this VPN Link Connection.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Whether to enable policy-based traffic selectors? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "policyBasedTrafficSelectorEnabled")]
    pub r#policy_based_traffic_selector_enabled: Box<Option<bool>>,
    /// The protocol used for this VPN Link Connection. Possible values are `IKEv1` and `IKEv2`. Defaults to `IKEv2`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// Should the rate limit be enabled? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ratelimitEnabled")]
    pub r#ratelimit_enabled: Box<Option<bool>>,
    /// Routing weight for this VPN Link Connection. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "routeWeight")]
    pub r#route_weight: Box<Option<i32>>,
    /// SharedKey for this VPN Link Connection.
    #[builder(into, default)]
    #[serde(rename = "sharedKey")]
    pub r#shared_key: Box<Option<String>>,
    /// The ID of the connected VPN Site Link. Changing this forces a new VPN Gateway Connection to be created.
    #[builder(into)]
    #[serde(rename = "vpnSiteLinkId")]
    pub r#vpn_site_link_id: Box<String>,
}
