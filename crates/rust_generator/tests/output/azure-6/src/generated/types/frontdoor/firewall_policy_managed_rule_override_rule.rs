#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyManagedRuleOverrideRule {
    /// The action to be applied when the rule matches. Possible values are `Allow`, `Block`, `Log`, or `Redirect`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Is the managed rule override enabled or disabled. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<super::super::types::frontdoor::FirewallPolicyManagedRuleOverrideRuleExclusion>>>,
    /// Identifier for the managed rule.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: Box<String>,
}
