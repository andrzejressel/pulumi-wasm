#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersHeader {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
