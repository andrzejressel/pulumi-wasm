#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAgent {
    /// A `extensions_allow_list` block as defined below.
    #[builder(into)]
    #[serde(rename = "extensionsAllowLists")]
    pub r#extensions_allow_lists: Box<Vec<super::super::types::arcmachine::GetAgentExtensionsAllowList>>,
    /// A `extensions_block_list` block as defined below.
    #[builder(into)]
    #[serde(rename = "extensionsBlockLists")]
    pub r#extensions_block_lists: Box<Vec<super::super::types::arcmachine::GetAgentExtensionsBlockList>>,
    /// Specifies whether the extension service is enabled or disabled.
    #[builder(into)]
    #[serde(rename = "extensionsEnabled")]
    pub r#extensions_enabled: Box<bool>,
    /// Specified whether the guest configuration service is enabled or disabled.
    #[builder(into)]
    #[serde(rename = "guestConfigurationEnabled")]
    pub r#guest_configuration_enabled: Box<bool>,
    /// Specifies the list of ports that the agent will be able to listen on.
    #[builder(into)]
    #[serde(rename = "incomingConnectionsPorts")]
    pub r#incoming_connections_ports: Box<Vec<String>>,
    /// List of service names which should not use the specified proxy server.
    #[builder(into)]
    #[serde(rename = "proxyBypasses")]
    pub r#proxy_bypasses: Box<Vec<String>>,
    /// Specifies the URL of the proxy to be used.
    #[builder(into)]
    #[serde(rename = "proxyUrl")]
    pub r#proxy_url: Box<String>,
}
