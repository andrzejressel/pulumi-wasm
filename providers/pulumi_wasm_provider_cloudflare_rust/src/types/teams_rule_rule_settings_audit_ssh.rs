#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsAuditSsh {
    #[serde(rename = "commandLogging")]
    pub r#command_logging: Box<bool>,
}
