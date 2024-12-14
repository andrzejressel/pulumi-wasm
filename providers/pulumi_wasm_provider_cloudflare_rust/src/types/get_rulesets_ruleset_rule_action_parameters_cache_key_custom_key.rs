#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey {
    /// Cookie parameters for the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie>>,
    /// Header parameters for the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHeader>>,
    /// Host parameters for the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "host")]
    pub r#host: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost>>,
    /// Query string parameters for the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyQueryString>>,
    /// User parameters for the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "user")]
    pub r#user: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser>>,
}
