#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsRuleRuleSettingsPayloadLog {
    /// Enable notification settings.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
