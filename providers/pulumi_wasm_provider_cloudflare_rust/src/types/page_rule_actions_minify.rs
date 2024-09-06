#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PageRuleActionsMinify {
    /// Whether CSS should be minified. Valid values are `"on"` or `"off"`.
    #[serde(rename = "css")]
    pub r#css: Box<String>,
    /// Whether HTML should be minified. Valid values are `"on"` or `"off"`.
    #[serde(rename = "html")]
    pub r#html: Box<String>,
    /// Whether Javascript should be minified. Valid values are `"on"` or `"off"`.
    #[serde(rename = "js")]
    pub r#js: Box<String>,
}
