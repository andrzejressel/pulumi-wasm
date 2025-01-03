#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementByteMatchStatementFieldToMatchJa3Fingerprint {
    #[builder(into)]
    #[serde(rename = "fallbackBehavior")]
    pub r#fallback_behavior: Box<String>,
}
