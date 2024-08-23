#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOverrides {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "categories")]
    pub r#categories:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesCategory>>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "rules")]
    pub r#rules:
        Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersOverridesRule>>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
