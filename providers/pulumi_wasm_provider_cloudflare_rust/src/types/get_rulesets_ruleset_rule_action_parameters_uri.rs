#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriPath>>,
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUriQuery>>,
}
