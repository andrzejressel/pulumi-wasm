#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersOrigin {
    /// Origin Hostname where request is sent.
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// Origin Port where request is sent.
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
