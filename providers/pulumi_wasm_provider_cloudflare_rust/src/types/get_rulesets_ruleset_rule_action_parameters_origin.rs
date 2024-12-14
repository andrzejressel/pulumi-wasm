#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersOrigin {
    /// Origin Hostname where request is sent.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// Origin Port where request is sent.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
