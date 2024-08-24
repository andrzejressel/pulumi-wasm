#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValue {
    /// Preserve query string for redirect URL.
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// Status code for which the edge TTL is applied.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Target URL for redirect.
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<Option<crate::types::RulesetRuleActionParametersFromValueTargetUrl>>,
}
