#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RulesetRuleActionParametersFromValue {
    /// Preserve query string for redirect URL.
    #[builder(into, default)]
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// Status code for redirect.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Target URL for redirect.
    #[builder(into, default)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<Option<super::types::RulesetRuleActionParametersFromValueTargetUrl>>,
}
