#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersFromList {
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
