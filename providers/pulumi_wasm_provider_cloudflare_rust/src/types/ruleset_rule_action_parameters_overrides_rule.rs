#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOverridesRule {
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "scoreThreshold")]
    pub r#score_threshold: Box<Option<i32>>,
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
}
