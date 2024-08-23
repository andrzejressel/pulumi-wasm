#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersUri {
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::RulesetRuleActionParametersUriPath>>,
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::RulesetRuleActionParametersUriQuery>>,
}
