#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    /// Status code for which the edge TTL is applied.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Status code range for which the edge TTL is applied.
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<
        Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>,
    >,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
}
