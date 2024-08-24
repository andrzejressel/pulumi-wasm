#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersAutominify {
    /// SSL minification.
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    /// HTML minification.
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    /// JS minification.
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}
