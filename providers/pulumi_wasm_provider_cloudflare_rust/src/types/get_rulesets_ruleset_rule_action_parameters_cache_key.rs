#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKey {
    /// Cache by device type. Conflicts with "custom_key.user.device_type".
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    /// Cache deception armor.
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    /// Custom key parameters for the request.
    #[serde(rename = "customKey")]
    pub r#custom_key:
        Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey>>,
    /// Ignore query strings order.
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}
