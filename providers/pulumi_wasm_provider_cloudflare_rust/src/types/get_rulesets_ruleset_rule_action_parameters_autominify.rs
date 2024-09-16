#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersAutominify {
    /// SSL minification.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "css")]
    pub r#css: Box<Option<bool>>,
    /// HTML minification.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "html")]
    pub r#html: Box<Option<bool>>,
    /// JS minification.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "js")]
    pub r#js: Box<Option<bool>>,
}
