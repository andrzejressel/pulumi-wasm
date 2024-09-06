#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetRulesetsRulesetRuleActionParametersFromValue {
    /// Preserve query string for redirect URL.
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// Status code for redirect.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Target URL for redirect.
    #[serde(rename = "targetUrl")]
    pub r#target_url:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromValueTargetUrl>>,
}
