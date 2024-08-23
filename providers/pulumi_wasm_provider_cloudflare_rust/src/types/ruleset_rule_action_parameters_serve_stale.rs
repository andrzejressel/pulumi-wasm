#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersServeStale {
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}
