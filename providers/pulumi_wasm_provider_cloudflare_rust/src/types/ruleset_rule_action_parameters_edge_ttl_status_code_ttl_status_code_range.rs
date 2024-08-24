#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    /// From status code.
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    /// To status code.
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}
