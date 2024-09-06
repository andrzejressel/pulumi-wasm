#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHost {
    /// Resolve hostname to IP address.
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}
