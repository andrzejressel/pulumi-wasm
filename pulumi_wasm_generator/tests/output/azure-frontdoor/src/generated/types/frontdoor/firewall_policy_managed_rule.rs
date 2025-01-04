#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyManagedRule {
    /// One or more `exclusion` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<super::super::types::frontdoor::FirewallPolicyManagedRuleExclusion>>>,
    /// One or more `override` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<super::super::types::frontdoor::FirewallPolicyManagedRuleOverride>>>,
    /// The name of the managed rule to use with this resource.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// The version on the managed rule to use with this resource.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
