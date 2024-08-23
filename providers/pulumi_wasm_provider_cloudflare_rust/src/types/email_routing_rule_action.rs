#[derive(serde::Serialize)]
pub struct EmailRoutingRuleAction {
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}
