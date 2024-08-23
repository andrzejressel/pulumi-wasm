#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllAction {
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
