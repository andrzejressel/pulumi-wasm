#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingRuleAction {
    /// Type of action. Available values: `forward`, `worker`, `drop`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Value to match on. Required for `type` of `literal`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}
