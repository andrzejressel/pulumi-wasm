#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourcePolicyAssignmentOverride {
    /// One or more `override_selector` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "selectors")]
    pub r#selectors: Box<Option<Vec<super::super::types::core::ResourcePolicyAssignmentOverrideSelector>>>,
    /// Specifies the value to override the policy property. Possible values for `policyEffect` override listed [policy effects](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/effects).
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}