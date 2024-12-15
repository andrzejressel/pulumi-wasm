#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKey {
    /// Cache by device type. Conflicts with "custom_key.user.device_type".
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    /// Cache deception armor.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    /// Custom key parameters for the request.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey>>,
    /// Ignore query strings order.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}
