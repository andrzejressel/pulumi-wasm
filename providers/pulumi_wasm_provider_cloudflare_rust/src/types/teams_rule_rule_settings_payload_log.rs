#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsPayloadLog {
    /// Enable notification settings.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
