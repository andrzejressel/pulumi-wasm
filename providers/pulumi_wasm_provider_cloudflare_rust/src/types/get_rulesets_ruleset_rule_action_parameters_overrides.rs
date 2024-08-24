#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverrides {
    /// Action to perform in the rule-level override. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of tag-based overrides.
    #[serde(rename = "categories")]
    pub r#categories: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesCategory>>>,
    /// Defines if the current ruleset-level override enables or disables the ruleset.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of rule-based overrides.
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesRule>>>,
    /// Sensitivity level to override for all ruleset rules. Available values: `default`, `medium`, `low`, `eoff`
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    /// Defines if the current ruleset-level override enables or disables the ruleset. Available values: `enabled`, `disabled`
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
