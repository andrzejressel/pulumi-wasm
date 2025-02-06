#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key. Conflicts with "include".
    #[builder(into, default)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// List of query string parameters to include in the custom key. Conflicts with "exclude".
    #[builder(into, default)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
