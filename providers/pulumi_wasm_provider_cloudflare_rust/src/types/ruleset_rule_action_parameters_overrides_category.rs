#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesCategory {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
