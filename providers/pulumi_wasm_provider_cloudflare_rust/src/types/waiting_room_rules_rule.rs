#[derive(serde::Serialize)]
pub struct WaitingRoomRulesRule {
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
