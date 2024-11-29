#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersServeStale {
    /// Disable stale while updating.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}
