#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParametersCacheKey {
    /// Cache by device type.
    #[builder(into, default)]
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<bool>>,
    /// Cache deception armor.
    #[builder(into, default)]
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<bool>>,
    /// Custom key parameters for the request.
    #[builder(into, default)]
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<super::types::RulesetRuleActionParametersCacheKeyCustomKey>>,
    /// Ignore query strings order.
    #[builder(into, default)]
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Box<Option<bool>>,
}
