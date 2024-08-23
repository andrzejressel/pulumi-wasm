#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKey {
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<crate::types::RulesetRuleActionParametersCacheKeyCustomKey>>,
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}
