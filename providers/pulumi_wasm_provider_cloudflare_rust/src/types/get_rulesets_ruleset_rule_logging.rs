#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleLogging {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
