#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RulesetRuleActionParametersOverridesRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Whether the rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Unique rule identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Anomaly score threshold to apply in the ruleset rule override. Only applicable to modsecurity-based rulesets.
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Box<Option<i32>>,
    /// Sensitivity level for a ruleset rule override.
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}
