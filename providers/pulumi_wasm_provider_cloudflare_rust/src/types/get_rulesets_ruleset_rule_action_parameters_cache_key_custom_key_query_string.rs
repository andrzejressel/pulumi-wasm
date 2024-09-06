#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key. Conflicts with "include".
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// List of query string parameters to include in the custom key. Conflicts with "exclude".
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
