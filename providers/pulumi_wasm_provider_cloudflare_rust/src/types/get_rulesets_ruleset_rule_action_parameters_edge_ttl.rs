#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersEdgeTtl {
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}
