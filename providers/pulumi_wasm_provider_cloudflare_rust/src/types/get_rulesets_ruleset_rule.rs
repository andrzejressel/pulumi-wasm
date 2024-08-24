#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of parameters that configure the behavior of the ruleset rule action.
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<crate::types::GetRulesetsRulesetRuleActionParameters>>,
    /// Brief summary of the ruleset rule and its intended use.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Whether the rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of parameters that configure exposed credential checks.
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check:
        Box<Option<crate::types::GetRulesetsRulesetRuleExposedCredentialCheck>>,
    /// Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// The ID of the Ruleset to target.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The most recent update to this rule.
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    /// List parameters to configure how the rule generates logs.
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<crate::types::GetRulesetsRulesetRuleLogging>>,
    /// List of parameters that configure HTTP rate limiting behaviour.
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<crate::types::GetRulesetsRulesetRuleRatelimit>>,
    /// Rule reference.
    #[serde(rename = "ref")]
    pub r#ref: Box<String>,
    /// Version of the ruleset to filter on.
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
