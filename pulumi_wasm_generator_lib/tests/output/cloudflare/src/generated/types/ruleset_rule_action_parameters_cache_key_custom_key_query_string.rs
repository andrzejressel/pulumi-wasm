#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyQueryString {
    /// List of query string parameters to exclude from the custom key.
    #[builder(into, default)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// List of query string parameters to include in the custom key.
    #[builder(into, default)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
