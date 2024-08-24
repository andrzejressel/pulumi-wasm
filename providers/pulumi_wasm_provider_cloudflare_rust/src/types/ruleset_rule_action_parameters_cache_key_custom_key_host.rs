#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHost {
    /// Resolve hostname to IP address.
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}
