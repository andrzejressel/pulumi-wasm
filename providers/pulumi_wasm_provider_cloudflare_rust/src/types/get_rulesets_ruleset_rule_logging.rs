#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleLogging {
    /// Override the default logging behavior when a rule is matched.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Override the default logging behavior when a rule is matched. Available values: `enabled`, `disabled`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
