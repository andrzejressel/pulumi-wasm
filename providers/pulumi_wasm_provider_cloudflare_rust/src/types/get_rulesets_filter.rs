#[derive(serde::Serialize)]
pub struct GetRulesetsFilter {
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "phase")]
    pub r#phase: Box<Option<String>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
