#[derive(serde::Serialize)]
pub struct UserAgentBlockingRuleConfiguration {
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
