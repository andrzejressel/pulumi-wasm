#[derive(serde::Serialize)]
pub struct DevicePostureRuleMatch {
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
}
