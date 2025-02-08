#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfile {
    /// Specifies a list of Azure Public IP Address IDs that can be used for Egress (Source) Network Address Translation.
    #[builder(into, default)]
    #[serde(rename = "egressNatIpAddressIds")]
    pub r#egress_nat_ip_address_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "egressNatIpAddresses")]
    pub r#egress_nat_ip_addresses: Box<Option<Vec<String>>>,
    /// Specifies a list of Azure Public IP Address IDs.
    #[builder(into)]
    #[serde(rename = "publicIpAddressIds")]
    pub r#public_ip_address_ids: Box<Vec<String>>,
    #[builder(into, default)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Option<Vec<String>>>,
    /// Specifies a list of trusted ranges to use for the Network.
    #[builder(into, default)]
    #[serde(rename = "trustedAddressRanges")]
    pub r#trusted_address_ranges: Box<Option<Vec<String>>>,
    /// A `vnet_configuration` block as defined below.
    #[builder(into)]
    #[serde(rename = "vnetConfiguration")]
    pub r#vnet_configuration: Box<super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfileVnetConfiguration>,
}
