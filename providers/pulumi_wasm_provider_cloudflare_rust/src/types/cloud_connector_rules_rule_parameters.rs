#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct CloudConnectorRulesRuleParameters {
    /// Host parameter for cloud connector rule
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
}
