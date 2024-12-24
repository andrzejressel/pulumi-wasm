#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
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
