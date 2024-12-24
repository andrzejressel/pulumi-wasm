#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersEdgeTtl {
    /// Default edge TTL.
    #[builder(into, default)]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the edge TTL. Available values: `override_origin`, `respect_origin`, `bypass_by_default`
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Edge TTL for the status codes.
    #[builder(into, default)]
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls: Box<Option<Vec<super::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}
