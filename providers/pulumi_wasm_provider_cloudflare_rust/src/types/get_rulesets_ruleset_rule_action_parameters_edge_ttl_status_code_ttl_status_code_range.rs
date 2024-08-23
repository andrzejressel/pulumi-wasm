#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}
