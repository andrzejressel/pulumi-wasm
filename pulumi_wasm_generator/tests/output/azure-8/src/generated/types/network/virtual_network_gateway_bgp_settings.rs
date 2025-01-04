#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkGatewayBgpSettings {
    /// The Autonomous System Number (ASN) to use as part of the BGP.
    #[builder(into, default)]
    #[serde(rename = "asn")]
    pub r#asn: Box<Option<i32>>,
    /// The weight added to routes which have been learned through BGP peering. Valid values can be between `0` and `100`.
    #[builder(into, default)]
    #[serde(rename = "peerWeight")]
    pub r#peer_weight: Box<Option<i32>>,
    /// A list of `peering_addresses` blocks as defined below. Only one `peering_addresses` block can be specified except when `active_active` of this Virtual Network Gateway is `true`.
    #[builder(into, default)]
    #[serde(rename = "peeringAddresses")]
    pub r#peering_addresses: Box<Option<Vec<super::super::types::network::VirtualNetworkGatewayBgpSettingsPeeringAddress>>>,
}
