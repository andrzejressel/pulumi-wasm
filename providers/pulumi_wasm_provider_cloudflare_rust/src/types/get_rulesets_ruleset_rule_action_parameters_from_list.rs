#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromList {
    /// Expression to use for the list lookup.
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Name of the ruleset.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
