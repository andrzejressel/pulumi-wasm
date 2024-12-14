#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHost {
    /// Resolve hostname to IP address.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}
