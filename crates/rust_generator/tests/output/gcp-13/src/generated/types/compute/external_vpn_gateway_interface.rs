#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ExternalVpnGatewayInterface {
    /// The numeric ID for this interface. Allowed values are based on the redundancy type
    /// of this external VPN gateway
    /// * `0 - SINGLE_IP_INTERNALLY_REDUNDANT`
    /// * `0, 1 - TWO_IPS_REDUNDANCY`
    /// * `0, 1, 2, 3 - FOUR_IPS_REDUNDANCY`
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<i32>>,
    /// IP address of the interface in the external VPN gateway.
    /// Only IPv4 is supported. This IP address can be either from
    /// your on-premise gateway or another Cloud provider's VPN gateway,
    /// it cannot be an IP address from Google Compute Engine.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// IPv6 address of the interface in the external VPN gateway. This IPv6
    /// address can be either from your on-premise gateway or another Cloud
    /// provider's VPN gateway, it cannot be an IP address from Google Compute
    /// Engine. Must specify an IPv6 address (not IPV4-mapped) using any format
    /// described in RFC 4291 (e.g. 2001:db8:0:0:2d9:51:0:0). The output format
    /// is RFC 5952 format (e.g. 2001:db8::2d9:51:0:0).
    #[builder(into, default)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
}
