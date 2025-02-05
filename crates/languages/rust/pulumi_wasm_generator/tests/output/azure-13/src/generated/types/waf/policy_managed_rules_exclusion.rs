#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyManagedRulesExclusion {
    /// One or more `excluded_rule_set` block defined below.
    #[builder(into, default)]
    #[serde(rename = "excludedRuleSet")]
    pub r#excluded_rule_set: Box<Option<super::super::types::waf::PolicyManagedRulesExclusionExcludedRuleSet>>,
    /// The name of the Match Variable. Possible values: `RequestArgKeys`, `RequestArgNames`, `RequestArgValues`, `RequestCookieKeys`, `RequestCookieNames`, `RequestCookieValues`, `RequestHeaderKeys`, `RequestHeaderNames`, `RequestHeaderValues`.
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: Box<String>,
    /// Describes field of the matchVariable collection.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Box<String>,
    /// Describes operator to be matched. Possible values: `Contains`, `EndsWith`, `Equals`, `EqualsAny`, `StartsWith`.
    #[builder(into)]
    #[serde(rename = "selectorMatchOperator")]
    pub r#selector_match_operator: Box<String>,
}
