#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnGatewayConnectionVpnLinkCustomBgpAddress {
    /// The custom bgp ip address which belongs to the IP Configuration.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// The ID of the IP Configuration which belongs to the VPN Gateway.
    #[builder(into)]
    #[serde(rename = "ipConfigurationId")]
    pub r#ip_configuration_id: Box<String>,
}
