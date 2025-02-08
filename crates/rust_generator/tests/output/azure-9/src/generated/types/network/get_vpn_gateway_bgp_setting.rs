#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVpnGatewayBgpSetting {
    /// The ASN of the BGP Speaker.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Box<i32>,
    /// The Address which should be used for the BGP Peering.
    #[builder(into)]
    #[serde(rename = "bgpPeeringAddress")]
    pub r#bgp_peering_address: Box<String>,
    /// an `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "instance0BgpPeeringAddresses")]
    pub r#instance_0_bgp_peering_addresses: Box<Vec<super::super::types::network::GetVpnGatewayBgpSettingInstance0BgpPeeringAddress>>,
    /// an `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "instance1BgpPeeringAddresses")]
    pub r#instance_1_bgp_peering_addresses: Box<Vec<super::super::types::network::GetVpnGatewayBgpSettingInstance1BgpPeeringAddress>>,
    /// The weight added to Routes learned from this BGP Speaker.
    #[builder(into)]
    #[serde(rename = "peerWeight")]
    pub r#peer_weight: Box<i32>,
}
