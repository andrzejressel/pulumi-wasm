#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingRuleMatcher {
    /// Field to match on. Required for `type` of `literal`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// Type of matcher. Available values: `literal`, `all`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Value to match on. Required for `type` of `literal`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
