#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersEdgeTtl {
    /// Default browser TTL. This value is required when override_origin is set
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL. Available values: `override_origin`, `respect_origin`, `bypass`
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Edge TTL for the status codes.
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls: Box<Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}
