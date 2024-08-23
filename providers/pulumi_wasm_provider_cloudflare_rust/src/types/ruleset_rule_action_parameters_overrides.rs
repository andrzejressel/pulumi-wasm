#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverrides {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "categories")]
    pub r#categories: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesCategory>>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<crate::types::RulesetRuleActionParametersOverridesRule>>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}
