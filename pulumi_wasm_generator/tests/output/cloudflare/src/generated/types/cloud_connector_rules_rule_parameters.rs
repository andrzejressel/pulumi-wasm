#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CloudConnectorRulesRuleParameters {
    /// Host parameter for cloud connector rule
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
}
