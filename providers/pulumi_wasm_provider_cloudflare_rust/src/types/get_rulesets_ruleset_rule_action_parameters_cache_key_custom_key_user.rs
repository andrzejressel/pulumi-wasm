#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser {
    /// Add device type to the custom key. Conflicts with "cache_key.cache_by_device_type".
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    /// Add geo data to the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    /// Add language data to the custom key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}
