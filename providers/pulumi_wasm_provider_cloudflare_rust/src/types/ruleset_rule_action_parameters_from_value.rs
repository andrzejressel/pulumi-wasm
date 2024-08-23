#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromValue {
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<Option<crate::types::RulesetRuleActionParametersFromValueTargetUrl>>,
}
