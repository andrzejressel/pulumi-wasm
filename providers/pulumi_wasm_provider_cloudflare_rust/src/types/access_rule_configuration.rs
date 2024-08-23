#[derive(serde::Serialize)]
pub struct AccessRuleConfiguration {
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
