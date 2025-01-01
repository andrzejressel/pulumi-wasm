#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorFirewallPolicyManagedRuleOverride {
    /// One or more `exclusion` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleOverrideExclusion>>>,
    /// The managed rule group to override.
    #[builder(into)]
    #[serde(rename = "ruleGroupName")]
    pub r#rule_group_name: Box<String>,
    /// One or more `rule` blocks as defined below. If none are specified, all of the rules in the group will be disabled.
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleOverrideRule>>>,
}
