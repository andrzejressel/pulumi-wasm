#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRule {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Box<Option<crate::types::GetRulesetsRulesetRuleActionParameters>>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check:
        Box<Option<crate::types::GetRulesetsRulesetRuleExposedCredentialCheck>>,
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Box<Option<String>>,
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<crate::types::GetRulesetsRulesetRuleLogging>>,
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Box<Option<crate::types::GetRulesetsRulesetRuleRatelimit>>,
    #[serde(rename = "ref")]
    pub r#ref: Box<String>,
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
