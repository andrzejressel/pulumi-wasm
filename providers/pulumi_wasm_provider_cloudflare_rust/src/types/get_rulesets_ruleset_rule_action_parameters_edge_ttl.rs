#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtl {
    /// Default edge TTL
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the edge TTL.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Edge TTL for the status codes.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}
