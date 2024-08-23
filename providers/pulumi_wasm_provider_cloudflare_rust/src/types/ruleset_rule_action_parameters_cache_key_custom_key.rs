#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKey {
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    #[serde(rename = "host")]
    pub r#host: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    #[serde(rename = "queryString")]
    pub r#query_string:
        Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}
