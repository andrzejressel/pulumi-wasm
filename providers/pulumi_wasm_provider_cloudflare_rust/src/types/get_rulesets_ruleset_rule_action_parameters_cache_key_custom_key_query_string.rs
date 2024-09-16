#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key. Conflicts with "include".
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// List of query string parameters to include in the custom key. Conflicts with "exclude".
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
