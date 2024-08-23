#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAutominify {
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}
