#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesCategory {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Tag name to apply the ruleset rule override to.
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    /// Defines if the current tag-level override enables or disables the ruleset rules with the specified tag.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
