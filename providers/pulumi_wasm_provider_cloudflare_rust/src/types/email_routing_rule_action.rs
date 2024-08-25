#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct EmailRoutingRuleAction {
    /// Type of action. Available values: `forward`, `worker`, `drop`
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Value to match on. Required for `type` of `literal`.
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}
