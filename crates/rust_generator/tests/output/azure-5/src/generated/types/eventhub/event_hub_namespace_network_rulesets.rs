#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventHubNamespaceNetworkRulesets {
    /// The default action to take when a rule is not matched. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<String>,
    /// One or more `ip_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<super::super::types::eventhub::EventHubNamespaceNetworkRulesetsIpRule>>>,
    /// Is public network access enabled for the EventHub Namespace? Defaults to `true`.
    /// 
    /// > **Note:** The public network access setting at the network rule sets level should be the same as it's at the namespace level.
    #[builder(into, default)]
    #[serde(rename = "publicNetworkAccessEnabled")]
    pub r#public_network_access_enabled: Box<Option<bool>>,
    /// Whether Trusted Microsoft Services are allowed to bypass firewall.
    #[builder(into, default)]
    #[serde(rename = "trustedServiceAccessEnabled")]
    pub r#trusted_service_access_enabled: Box<Option<bool>>,
    /// One or more `virtual_network_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkRules")]
    pub r#virtual_network_rules: Box<Option<Vec<super::super::types::eventhub::EventHubNamespaceNetworkRulesetsVirtualNetworkRule>>>,
}
