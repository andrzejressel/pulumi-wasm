#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<
        Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>,
    >,
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
}
