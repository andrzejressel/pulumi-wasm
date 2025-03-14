#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
