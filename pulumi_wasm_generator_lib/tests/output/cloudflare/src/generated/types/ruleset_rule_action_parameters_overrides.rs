#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersOverrides {
    /// Action to perform in the rule-level override. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of tag-based overrides.
    #[builder(into, default)]
    #[serde(rename = "categories")]
    pub r#categories: Box<Option<Vec<super::types::RulesetRuleActionParametersOverridesCategory>>>,
    /// Defines if the current ruleset-level override enables or disables the ruleset.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of rule-based overrides.
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<super::types::RulesetRuleActionParametersOverridesRule>>>,
    /// Sensitivity level to override for all ruleset rules. Available values: `default`, `medium`, `low`, `eoff`.
    #[builder(into, default)]
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}