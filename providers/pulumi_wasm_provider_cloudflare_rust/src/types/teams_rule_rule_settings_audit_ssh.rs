#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsRuleRuleSettingsAuditSsh {
    /// Log all SSH commands.
    #[serde(rename = "commandLogging")]
    pub r#command_logging: Box<bool>,
}
