#[derive(serde::Serialize)]
pub struct RulesetRuleLogging {
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
