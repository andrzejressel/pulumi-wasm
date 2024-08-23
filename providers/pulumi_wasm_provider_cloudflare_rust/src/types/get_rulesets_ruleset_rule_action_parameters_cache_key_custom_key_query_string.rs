#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
