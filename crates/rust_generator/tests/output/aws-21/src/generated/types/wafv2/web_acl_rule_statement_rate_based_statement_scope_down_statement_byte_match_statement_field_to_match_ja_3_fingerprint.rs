#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementRateBasedStatementScopeDownStatementByteMatchStatementFieldToMatchJa3Fingerprint {
    /// The match status to assign to the web request if the request doesn't have a JA3 fingerprint. Valid values include: `MATCH` or `NO_MATCH`.
    #[builder(into)]
    #[serde(rename = "fallbackBehavior")]
    pub r#fallback_behavior: Box<String>,
}
