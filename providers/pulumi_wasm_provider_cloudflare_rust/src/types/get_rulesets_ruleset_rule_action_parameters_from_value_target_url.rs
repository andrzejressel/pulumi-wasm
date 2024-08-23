#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersFromValueTargetUrl {
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
