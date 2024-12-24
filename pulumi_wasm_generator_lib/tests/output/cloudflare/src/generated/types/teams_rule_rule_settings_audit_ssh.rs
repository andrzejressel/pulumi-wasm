#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsRuleRuleSettingsAuditSsh {
    /// Log all SSH commands.
    #[builder(into)]
    #[serde(rename = "commandLogging")]
    pub r#command_logging: Box<bool>,
}
