#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyManagedRulesManagedRuleSet {
    /// One or more `rule_group_override` block defined below.
    #[builder(into, default)]
    #[serde(rename = "ruleGroupOverrides")]
    pub r#rule_group_overrides: Box<Option<Vec<super::super::types::waf::PolicyManagedRulesManagedRuleSetRuleGroupOverride>>>,
    /// The rule set type. Possible values: `Microsoft_BotManagerRuleSet`, `Microsoft_DefaultRuleSet` and `OWASP`. Defaults to `OWASP`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    /// The rule set version. Possible values: `0.1`, `1.0`, `1.1`, `2.1`, `2.2.9`, `3.0`, `3.1` and `3.2`.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
