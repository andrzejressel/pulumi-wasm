#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersServeStale {
    /// Disable stale while updating.
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}
