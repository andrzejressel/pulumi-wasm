#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// List of parameters that configure the behavior of the ruleset rule action.
    #[builder(into, default)]
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<super::types::GetRulesetsRulesetRuleActionParameters>>,
    /// Brief summary of the ruleset rule and its intended use.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Whether the rule is active.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of parameters that configure exposed credential checks.
    #[builder(into, default)]
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check: Box<Option<super::types::GetRulesetsRulesetRuleExposedCredentialCheck>>,
    /// Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// Unique rule identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The most recent update to this rule.
    #[builder(into, default)]
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    /// List parameters to configure how the rule generates logs.
    #[builder(into, default)]
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<super::types::GetRulesetsRulesetRuleLogging>>,
    /// List of parameters that configure HTTP rate limiting behaviour.
    #[builder(into, default)]
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<super::types::GetRulesetsRulesetRuleRatelimit>>,
    /// Rule reference.
    #[builder(into)]
    #[serde(rename = "ref")]
    pub r#ref: Box<String>,
    /// Version of the ruleset to deploy.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
