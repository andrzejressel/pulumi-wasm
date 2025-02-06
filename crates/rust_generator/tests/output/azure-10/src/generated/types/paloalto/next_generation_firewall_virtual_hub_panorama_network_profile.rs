#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NextGenerationFirewallVirtualHubPanoramaNetworkProfile {
    #[builder(into, default)]
    #[serde(rename = "egressNatIpAddressIds")]
    pub r#egress_nat_ip_address_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "egressNatIpAddresses")]
    pub r#egress_nat_ip_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "ipOfTrustForUserDefinedRoutes")]
    pub r#ip_of_trust_for_user_defined_routes: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "networkVirtualApplianceId")]
    pub r#network_virtual_appliance_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "publicIpAddressIds")]
    pub r#public_ip_address_ids: Box<Vec<String>>,
    #[builder(into, default)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "trustedAddressRanges")]
    pub r#trusted_address_ranges: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "trustedSubnetId")]
    pub r#trusted_subnet_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "untrustedSubnetId")]
    pub r#untrusted_subnet_id: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "virtualHubId")]
    pub r#virtual_hub_id: Box<String>,
}
