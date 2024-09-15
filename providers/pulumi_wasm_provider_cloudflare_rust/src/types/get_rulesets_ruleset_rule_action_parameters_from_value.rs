#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersFromValue {
    /// Preserve query string for redirect URL.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// Status code for redirect.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Target URL for redirect.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "targetUrl")]
    pub r#target_url:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromValueTargetUrl>>,
}
