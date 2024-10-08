#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersBrowserTtl {
    /// Default browser TTL.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    /// Mode of the browser TTL.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
