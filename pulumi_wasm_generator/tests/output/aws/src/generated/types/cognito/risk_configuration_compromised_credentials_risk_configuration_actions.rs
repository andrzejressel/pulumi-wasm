#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RiskConfigurationCompromisedCredentialsRiskConfigurationActions {
    /// The event action. Valid values are `BLOCK` or `NO_ACTION`.
    #[builder(into)]
    #[serde(rename = "eventAction")]
    pub r#event_action: Box<String>,
}