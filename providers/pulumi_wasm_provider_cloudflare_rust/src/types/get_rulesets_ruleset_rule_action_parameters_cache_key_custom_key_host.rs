#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyHost {
    /// Resolve hostname to IP address.
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}
