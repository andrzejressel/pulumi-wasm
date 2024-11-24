#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// List of query string parameters to include in the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
