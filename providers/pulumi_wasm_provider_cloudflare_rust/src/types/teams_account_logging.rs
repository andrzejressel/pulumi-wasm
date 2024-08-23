#[derive(serde::Serialize)]
pub struct TeamsAccountLogging {
    #[serde(rename = "redactPii")]
    pub r#redact_pii: Box<bool>,
    #[serde(rename = "settingsByRuleType")]
    pub r#settings_by_rule_type: Box<crate::types::TeamsAccountLoggingSettingsByRuleType>,
}
