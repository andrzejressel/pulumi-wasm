#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsPayloadLog {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
