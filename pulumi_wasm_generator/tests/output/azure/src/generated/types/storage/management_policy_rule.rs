#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagementPolicyRule {
    /// An `actions` block as documented below.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<super::super::types::storage::ManagementPolicyRuleActions>,
    /// Boolean to specify whether the rule is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A `filters` block as documented below.
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Box<super::super::types::storage::ManagementPolicyRuleFilters>,
    /// The name of the rule. Rule name is case-sensitive. It must be unique within a policy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}