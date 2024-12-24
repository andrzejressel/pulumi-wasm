#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersBrowserTtl {
    /// Default browser TTL.
    #[builder(into, default)]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
