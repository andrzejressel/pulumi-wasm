#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct EmailRoutingRuleMatcher {
    /// Field to match on. Required for `type` of `literal`.
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// Type of matcher. Available values: `literal`, `all`
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Value to match on. Required for `type` of `literal`.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
