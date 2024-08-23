#[derive(serde::Serialize)]
pub struct GetRulesetsRuleset {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "phase")]
    pub r#phase: Box<String>,
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::GetRulesetsRulesetRule>>>,
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
