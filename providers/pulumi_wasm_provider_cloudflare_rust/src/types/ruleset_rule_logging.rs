#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleLogging {
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
