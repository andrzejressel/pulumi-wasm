#[derive(serde::Serialize)]
pub struct GetDevicePostureRulesRule {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
