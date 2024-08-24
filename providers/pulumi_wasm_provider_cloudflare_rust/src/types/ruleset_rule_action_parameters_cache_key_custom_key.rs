#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKey {
    /// Cookie parameters for the custom key.
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    /// Header parameters for the custom key.
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    /// Host parameters for the custom key.
    #[serde(rename = "host")]
    pub r#host: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    /// Query string parameters for the custom key.
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    /// User parameters for the custom key.
    #[serde(rename = "user")]
    pub r#user: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}
