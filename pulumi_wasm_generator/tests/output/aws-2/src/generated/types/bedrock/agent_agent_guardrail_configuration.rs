#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentAgentGuardrailConfiguration {
    /// Unique identifier of the guardrail.
    #[builder(into)]
    #[serde(rename = "guardrailIdentifier")]
    pub r#guardrail_identifier: Box<String>,
    /// Version of the guardrail.
    #[builder(into)]
    #[serde(rename = "guardrailVersion")]
    pub r#guardrail_version: Box<String>,
}
