#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOrigin {
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
