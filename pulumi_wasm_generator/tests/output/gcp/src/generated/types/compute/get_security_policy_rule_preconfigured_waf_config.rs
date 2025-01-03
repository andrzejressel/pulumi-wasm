#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyRulePreconfiguredWafConfig {
    /// An exclusion to apply during preconfigured WAF evaluation.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusion>>,
}
