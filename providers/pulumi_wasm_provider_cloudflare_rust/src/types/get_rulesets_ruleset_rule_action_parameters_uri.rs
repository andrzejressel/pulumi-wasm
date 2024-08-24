#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    /// URI path configuration when performing a URL rewrite.
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriPath>>,
    /// Query string configuration when performing a URL rewrite.
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriQuery>>,
}
