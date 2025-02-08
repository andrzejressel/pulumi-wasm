#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLocalNetworkGatewayBgpSetting {
    /// The BGP speaker's ASN.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Box<i32>,
    /// The BGP peering address and BGP identifier of this BGP speaker.
    #[builder(into)]
    #[serde(rename = "bgpPeeringAddress")]
    pub r#bgp_peering_address: Box<String>,
    /// The weight added to routes learned from this BGP speaker.
    #[builder(into)]
    #[serde(rename = "peerWeight")]
    pub r#peer_weight: Box<i32>,
}
