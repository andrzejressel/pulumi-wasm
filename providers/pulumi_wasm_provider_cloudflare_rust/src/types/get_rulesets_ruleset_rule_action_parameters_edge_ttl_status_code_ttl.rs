#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>>,
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
