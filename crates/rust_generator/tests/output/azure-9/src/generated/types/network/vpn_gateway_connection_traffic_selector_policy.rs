#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VpnGatewayConnectionTrafficSelectorPolicy {
    /// A list of local address spaces in CIDR format for this VPN Gateway Connection.
    #[builder(into)]
    #[serde(rename = "localAddressRanges")]
    pub r#local_address_ranges: Box<Vec<String>>,
    /// A list of remote address spaces in CIDR format for this VPN Gateway Connection.
    #[builder(into)]
    #[serde(rename = "remoteAddressRanges")]
    pub r#remote_address_ranges: Box<Vec<String>>,
}
