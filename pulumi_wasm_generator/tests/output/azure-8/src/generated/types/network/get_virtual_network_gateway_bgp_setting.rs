#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNetworkGatewayBgpSetting {
    /// The Autonomous System Number (ASN) to use as part of the BGP.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Box<i32>,
    /// The weight added to routes which have been learned
    /// through BGP peering.
    #[builder(into)]
    #[serde(rename = "peerWeight")]
    pub r#peer_weight: Box<i32>,
    /// The BGP peer IP address of the virtual network
    /// gateway. This address is needed to configure the created gateway as a BGP Peer
    /// on the on-premises VPN devices.
    #[builder(into)]
    #[serde(rename = "peeringAddress")]
    pub r#peering_address: Box<String>,
}
