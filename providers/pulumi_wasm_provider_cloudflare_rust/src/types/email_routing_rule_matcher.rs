#[derive(serde::Serialize)]
pub struct EmailRoutingRuleMatcher {
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
