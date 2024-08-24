#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverridesRule {
    /// Action to perform in the rule-level override. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Defines if the current rule-level override enables or disables the rule.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The ID of the Ruleset to target.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Anomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets.
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Box<Option<i32>>,
    /// Sensitivity level for a ruleset rule override.
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    /// Defines if the current rule-level override enables or disables the rule. Available values: `enabled`, `disabled`
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
