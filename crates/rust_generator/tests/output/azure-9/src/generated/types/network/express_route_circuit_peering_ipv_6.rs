#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExpressRouteCircuitPeeringIpv6 {
    /// A boolean value indicating whether the IPv6 peering is enabled. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// A `microsoft_peering` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "microsoftPeering")]
    pub r#microsoft_peering: Box<Option<super::super::types::network::ExpressRouteCircuitPeeringIpv6MicrosoftPeering>>,
    /// A subnet for the primary link.
    #[builder(into)]
    #[serde(rename = "primaryPeerAddressPrefix")]
    pub r#primary_peer_address_prefix: Box<String>,
    /// The ID of the Route Filter. Only available when `peering_type` is set to `MicrosoftPeering`.
    /// 
    /// > **NOTE:** `ipv6` can be specified when `peering_type` is `MicrosoftPeering` or `AzurePrivatePeering`
    #[builder(into, default)]
    #[serde(rename = "routeFilterId")]
    pub r#route_filter_id: Box<Option<String>>,
    /// A subnet for the secondary link.
    #[builder(into)]
    #[serde(rename = "secondaryPeerAddressPrefix")]
    pub r#secondary_peer_address_prefix: Box<String>,
}
