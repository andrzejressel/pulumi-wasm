#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NextGenerationFirewallVirtualNetworkPanoramaDnsSettings {
    #[builder(into, default)]
    #[serde(rename = "azureDnsServers")]
    pub r#azure_dns_servers: Box<Option<Vec<String>>>,
    /// Specifies a list of DNS servers to use. Conflicts with `dns_settings[0].use_azure_dns`.
    #[builder(into, default)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    /// Should the Firewall use Azure Supplied DNS servers. Conflicts with `dns_settings[0].dns_servers`. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "useAzureDns")]
    pub r#use_azure_dns: Box<Option<bool>>,
}
