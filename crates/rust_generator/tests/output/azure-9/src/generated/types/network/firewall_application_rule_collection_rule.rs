#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FirewallApplicationRuleCollectionRule {
    /// Specifies a description for the rule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A list of FQDN tags. Possible values are `AppServiceEnvironment`, `AzureBackup`, `AzureKubernetesService`, `HDInsight`, `MicrosoftActiveProtectionService`, `WindowsDiagnostics`, `WindowsUpdate` and `WindowsVirtualDesktop`.
    #[builder(into, default)]
    #[serde(rename = "fqdnTags")]
    pub r#fqdn_tags: Box<Option<Vec<String>>>,
    /// Specifies the name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `protocol` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Option<Vec<super::super::types::network::FirewallApplicationRuleCollectionRuleProtocol>>>,
    /// A list of source IP addresses and/or IP ranges.
    #[builder(into, default)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Box<Option<Vec<String>>>,
    /// A list of source IP Group IDs for the rule.
    /// 
    /// > **NOTE** At least one of `source_addresses` and `source_ip_groups` must be specified for a rule.
    #[builder(into, default)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Box<Option<Vec<String>>>,
    /// A list of FQDNs.
    #[builder(into, default)]
    #[serde(rename = "targetFqdns")]
    pub r#target_fqdns: Box<Option<Vec<String>>>,
}
