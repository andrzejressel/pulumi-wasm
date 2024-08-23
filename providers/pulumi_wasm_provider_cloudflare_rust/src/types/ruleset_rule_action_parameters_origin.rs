#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersOrigin {
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
