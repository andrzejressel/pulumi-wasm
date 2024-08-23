#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
