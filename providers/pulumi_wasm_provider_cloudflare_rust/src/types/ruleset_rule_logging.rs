#[derive(serde::Serialize)]
pub struct RulesetRuleLogging {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
