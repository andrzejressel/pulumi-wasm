#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetRulesetsRulesetRuleLogging {
    /// Override the default logging behavior when a rule is matched.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Override the default logging behavior when a rule is matched. Available values: `enabled`, `disabled`
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
