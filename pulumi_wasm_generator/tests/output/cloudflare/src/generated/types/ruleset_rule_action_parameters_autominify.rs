#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RulesetRuleActionParametersAutominify {
    /// CSS minification.
    #[builder(into, default)]
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    /// HTML minification.
    #[builder(into, default)]
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    /// JS minification.
    #[builder(into, default)]
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}