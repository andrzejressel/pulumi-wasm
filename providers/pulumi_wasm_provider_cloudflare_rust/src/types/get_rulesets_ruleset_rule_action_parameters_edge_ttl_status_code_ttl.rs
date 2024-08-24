#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    /// Status code for which the edge TTL is applied. Conflicts with "status_code_range".
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Status code range for which the edge TTL is applied. Conflicts with "status_code".
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
