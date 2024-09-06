#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RulesetRuleActionParametersOrigin {
    /// Host parameters for the custom key.
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// Origin Port where request is sent.
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
