#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersOverrides {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of tag-based overrides.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "categories")]
    pub r#categories: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesCategory>>>,
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of rule-based overrides.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesRule>>>,
    /// Sensitivity level for a ruleset rule override.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}
