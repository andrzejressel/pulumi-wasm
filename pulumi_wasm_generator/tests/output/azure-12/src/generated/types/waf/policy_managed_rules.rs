#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyManagedRules {
    /// One or more `exclusion` block defined below.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<super::super::types::waf::PolicyManagedRulesExclusion>>>,
    /// One or more `managed_rule_set` block defined below.
    #[builder(into)]
    #[serde(rename = "managedRuleSets")]
    pub r#managed_rule_sets: Box<Vec<super::super::types::waf::PolicyManagedRulesManagedRuleSet>>,
}
