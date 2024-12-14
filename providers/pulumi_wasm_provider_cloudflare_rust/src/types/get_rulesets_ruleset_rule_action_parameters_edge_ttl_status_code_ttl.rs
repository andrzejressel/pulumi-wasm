#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl {
    /// Status code for which the edge TTL is applied. Conflicts with "status_code_range".
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Status code range for which the edge TTL is applied. Conflicts with "status_code".
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCodeRanges")]
    pub r#status_code_ranges: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange>>>,
    /// Status code edge TTL value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
