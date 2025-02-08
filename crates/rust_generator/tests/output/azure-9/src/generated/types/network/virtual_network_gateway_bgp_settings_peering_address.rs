#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualNetworkGatewayBgpSettingsPeeringAddress {
    /// A list of Azure custom APIPA addresses assigned to the BGP peer of the Virtual Network Gateway.
    /// 
    /// > **Note:** The valid range for the reserved APIPA address in Azure Public is from `169.254.21.0` to `169.254.22.255`.
    #[builder(into, default)]
    #[serde(rename = "apipaAddresses")]
    pub r#apipa_addresses: Box<Option<Vec<String>>>,
    /// A list of peering address assigned to the BGP peer of the Virtual Network Gateway.
    #[builder(into, default)]
    #[serde(rename = "defaultAddresses")]
    pub r#default_addresses: Box<Option<Vec<String>>>,
    /// The name of the IP configuration of this Virtual Network Gateway. In case there are multiple `ip_configuration` blocks defined, this property is **required** to specify.
    #[builder(into, default)]
    #[serde(rename = "ipConfigurationName")]
    pub r#ip_configuration_name: Box<Option<String>>,
    /// A list of tunnel IP addresses assigned to the BGP peer of the Virtual Network Gateway.
    #[builder(into, default)]
    #[serde(rename = "tunnelIpAddresses")]
    pub r#tunnel_ip_addresses: Box<Option<Vec<String>>>,
}
