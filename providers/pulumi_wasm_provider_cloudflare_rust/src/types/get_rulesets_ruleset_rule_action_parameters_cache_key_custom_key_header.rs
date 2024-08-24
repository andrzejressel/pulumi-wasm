#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader {
    /// List of headers to check for presence in the custom key.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Exclude the origin header from the custom key.
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    /// List of headers to include in the custom key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
