#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersBrowserTtl {
    #[serde(rename = "default")]
    pub r#default: Box<Option<i32>>,
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
