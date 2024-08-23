#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersServeStale {
    #[serde(rename = "disableStaleWhileUpdating")]
    pub r#disable_stale_while_updating: Box<Option<bool>>,
}
