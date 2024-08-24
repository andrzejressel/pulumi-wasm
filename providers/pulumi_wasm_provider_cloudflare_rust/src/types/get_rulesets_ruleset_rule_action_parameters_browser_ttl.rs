#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersBrowserTtl {
    /// Default browser TTL.
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL.
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
