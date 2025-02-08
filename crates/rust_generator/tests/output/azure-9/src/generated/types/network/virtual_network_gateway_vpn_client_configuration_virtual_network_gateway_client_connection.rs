#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNetworkGatewayVpnClientConfigurationVirtualNetworkGatewayClientConnection {
    /// A list of address prefixes for P2S VPN Client.
    #[builder(into)]
    #[serde(rename = "addressPrefixes")]
    pub r#address_prefixes: Box<Vec<String>>,
    /// The name of the Virtual Network Gateway Client Connection.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A list of names of Virtual Network Gateway Policy Groups.
    #[builder(into)]
    #[serde(rename = "policyGroupNames")]
    pub r#policy_group_names: Box<Vec<String>>,
}
