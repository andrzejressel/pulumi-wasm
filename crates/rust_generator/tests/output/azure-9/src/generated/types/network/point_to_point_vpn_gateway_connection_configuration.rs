#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PointToPointVpnGatewayConnectionConfiguration {
    /// Should Internet Security be enabled to secure internet traffic? Changing this forces a new resource to be created. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "internetSecurityEnabled")]
    pub r#internet_security_enabled: Box<Option<bool>>,
    /// The Name which should be used for this Connection Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `route` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "route")]
    pub r#route: Box<Option<super::super::types::network::PointToPointVpnGatewayConnectionConfigurationRoute>>,
    /// A `vpn_client_address_pool` block as defined below.
    #[builder(into)]
    #[serde(rename = "vpnClientAddressPool")]
    pub r#vpn_client_address_pool: Box<super::super::types::network::PointToPointVpnGatewayConnectionConfigurationVpnClientAddressPool>,
}
