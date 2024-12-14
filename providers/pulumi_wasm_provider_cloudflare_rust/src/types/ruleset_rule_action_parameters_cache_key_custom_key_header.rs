#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHeader {
    /// List of headers to check for presence in the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Dictionary of headers mapping to lists of values to check for presence in the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "contains")]
    pub r#contains: Box<Option<std::collections::HashMap<String, Vec<String>>>>,
    /// Exclude the origin header from the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    /// List of headers to include in the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
