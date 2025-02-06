#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PageRuleActionsMinify {
    /// Whether CSS should be minified. Valid values are `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    /// Whether HTML should be minified. Valid values are `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    /// Whether Javascript should be minified. Valid values are `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}
