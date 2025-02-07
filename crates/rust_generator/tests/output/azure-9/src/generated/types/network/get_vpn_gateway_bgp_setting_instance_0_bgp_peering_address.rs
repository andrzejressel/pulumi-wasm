#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpnGatewayBgpSettingInstance0BgpPeeringAddress {
    /// A list of custom BGP peering addresses to assigned to this instance.
    #[builder(into)]
    #[serde(rename = "customIps")]
    pub r#custom_ips: Box<Vec<String>>,
    /// The list of default BGP peering addresses which belong to the pre-defined VPN Gateway IP configuration.
    #[builder(into)]
    #[serde(rename = "defaultIps")]
    pub r#default_ips: Box<Vec<String>>,
    /// The pre-defined id of VPN Gateway IP Configuration.
    #[builder(into)]
    #[serde(rename = "ipConfigurationId")]
    pub r#ip_configuration_id: Box<String>,
    /// The list of tunnel public IP addresses which belong to the pre-defined VPN Gateway IP configuration.
    #[builder(into)]
    #[serde(rename = "tunnelIps")]
    pub r#tunnel_ips: Box<Vec<String>>,
}
