#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtlStatusCodeRange {
    /// From status code.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    /// To status code.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}
