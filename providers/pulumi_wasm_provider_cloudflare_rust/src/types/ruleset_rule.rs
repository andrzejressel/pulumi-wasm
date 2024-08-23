#[derive(serde::Serialize)]
pub struct RulesetRule {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<crate::types::RulesetRuleActionParameters>>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check: Box<Option<crate::types::RulesetRuleExposedCredentialCheck>>,
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<crate::types::RulesetRuleLogging>>,
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<crate::types::RulesetRuleRatelimit>>,
    #[serde(rename = "ref")]
    pub r#ref: Box<Option<String>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
