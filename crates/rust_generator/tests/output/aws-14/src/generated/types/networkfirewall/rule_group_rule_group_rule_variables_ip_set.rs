#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupRuleVariablesIpSet {
    /// A configuration block that defines a set of IP addresses. See IP Set below for details.
    #[builder(into)]
    #[serde(rename = "ipSet")]
    pub r#ip_set: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRuleVariablesIpSetIpSet>,
    /// A unique alphanumeric string to identify the `ip_set`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
