#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyUser {
    /// Add device type to the custom key.
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    /// Add geo data to the custom key.
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    /// Add language data to the custom key.
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}
