#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey {
    #[serde(rename = "cookie")]
    pub r#cookie:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    #[serde(rename = "header")]
    pub r#header:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    #[serde(rename = "host")]
    pub r#host:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    #[serde(rename = "queryString")]
    pub r#query_string: Box<
        Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString>,
    >,
    #[serde(rename = "user")]
    pub r#user:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}
