#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResolverFirewallRulesFirewallRule {
    /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The DNS record's type.
    #[builder(into)]
    #[serde(rename = "blockOverrideDnsType")]
    pub r#block_override_dns_type: Box<String>,
    /// The custom DNS record to send back in response to the query.
    #[builder(into)]
    #[serde(rename = "blockOverrideDomain")]
    pub r#block_override_domain: Box<String>,
    /// The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record.
    #[builder(into)]
    #[serde(rename = "blockOverrideTtl")]
    pub r#block_override_ttl: Box<i32>,
    /// The way that you want DNS Firewall to block the request.
    #[builder(into)]
    #[serde(rename = "blockResponse")]
    pub r#block_response: Box<String>,
    /// The date and time that the rule was created, in Unix time format and Coordinated Universal Time (UTC).
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Box<String>,
    /// A unique string defined by you to identify the request.
    #[builder(into)]
    #[serde(rename = "creatorRequestId")]
    pub r#creator_request_id: Box<String>,
    /// The ID of the domain list that's used in the rule.
    #[builder(into)]
    #[serde(rename = "firewallDomainListId")]
    pub r#firewall_domain_list_id: Box<String>,
    /// The unique identifier of the firewall rule group that you want to retrieve the rules for.
    #[builder(into)]
    #[serde(rename = "firewallRuleGroupId")]
    pub r#firewall_rule_group_id: Box<String>,
    /// The date and time that the rule was last modified, in Unix time format and Coordinated Universal Time (UTC).
    #[builder(into)]
    #[serde(rename = "modificationTime")]
    pub r#modification_time: Box<String>,
    /// The name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The setting that determines the processing order of the rules in a rule group.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
}
