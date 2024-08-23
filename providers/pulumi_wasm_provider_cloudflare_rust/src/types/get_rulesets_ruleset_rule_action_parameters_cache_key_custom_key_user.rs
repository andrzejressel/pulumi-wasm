#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser {
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}
