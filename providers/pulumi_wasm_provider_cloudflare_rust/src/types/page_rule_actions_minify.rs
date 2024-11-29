#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
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
