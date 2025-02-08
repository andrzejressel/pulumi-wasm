#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFirewallPolicyDn {
    #[builder(into)]
    #[serde(rename = "networkRuleFqdnEnabled")]
    pub r#network_rule_fqdn_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "proxyEnabled")]
    pub r#proxy_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "servers")]
    pub r#servers: Box<Vec<String>>,
}
