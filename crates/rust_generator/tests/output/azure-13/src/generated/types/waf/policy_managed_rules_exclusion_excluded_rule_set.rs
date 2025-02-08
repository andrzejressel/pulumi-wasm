#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyManagedRulesExclusionExcludedRuleSet {
    /// One or more `rule_group` block defined below.
    #[builder(into, default)]
    #[serde(rename = "ruleGroups")]
    pub r#rule_groups: Box<Option<Vec<super::super::types::waf::PolicyManagedRulesExclusionExcludedRuleSetRuleGroup>>>,
    /// The rule set type. Possible values are `Microsoft_DefaultRuleSet`, `Microsoft_BotManagerRuleSet` and `OWASP`. Defaults to `OWASP`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// The rule set version. Possible values are `1.0`, `1.1` (for rule set type `Microsoft_BotManagerRuleSet`), `2.1` (for rule set type `Microsoft_DefaultRuleSet`) and `3.2` (for rule set type `OWASP`). Defaults to `3.2`.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
