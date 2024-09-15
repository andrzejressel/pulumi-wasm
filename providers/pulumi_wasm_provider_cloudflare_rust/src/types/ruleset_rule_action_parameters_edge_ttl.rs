#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersEdgeTtl {
    /// Default browser TTL. This value is required when override_origin is set
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL. Available values: `override_origin`, `respect_origin`, `bypass`
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Edge TTL for the status codes.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls:
        Box<Option<Vec<crate::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtl>>>,
}
