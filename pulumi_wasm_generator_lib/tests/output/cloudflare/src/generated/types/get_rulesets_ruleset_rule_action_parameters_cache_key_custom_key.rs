#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey {
    /// Cookie parameters for the custom key.
    #[builder(into, default)]
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    /// Header parameters for the custom key.
    #[builder(into, default)]
    #[serde(rename = "header")]
    pub r#header: Box<Option<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    /// Host parameters for the custom key.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    /// Query string parameters for the custom key.
    #[builder(into, default)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    /// User parameters for the custom key.
    #[builder(into, default)]
    #[serde(rename = "user")]
    pub r#user: Box<Option<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}