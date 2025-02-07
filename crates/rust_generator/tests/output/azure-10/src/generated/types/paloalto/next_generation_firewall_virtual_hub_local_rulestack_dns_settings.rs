#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NextGenerationFirewallVirtualHubLocalRulestackDnsSettings {
    #[builder(into, default)]
    #[serde(rename = "azureDnsServers")]
    pub r#azure_dns_servers: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "useAzureDns")]
    pub r#use_azure_dns: Box<Option<bool>>,
}
