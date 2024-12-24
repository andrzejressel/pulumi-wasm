#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CloudConnectorRulesRuleParameters {
    /// Host parameter for cloud connector rule
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
}
