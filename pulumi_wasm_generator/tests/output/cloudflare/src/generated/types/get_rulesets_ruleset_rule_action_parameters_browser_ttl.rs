#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRulesetsRulesetRuleActionParametersBrowserTtl {
    /// Default browser TTL.
    #[builder(into, default)]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
