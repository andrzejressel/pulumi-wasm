#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersBrowserTtl {
    /// Default browser TTL. This value is required when override_origin is set
    #[builder(into, default)]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL. Available values: `override_origin`, `respect_origin`, `bypass`
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
