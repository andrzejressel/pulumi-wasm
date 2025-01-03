#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuardrailSensitiveInformationPolicyConfigPiiEntitiesConfig {
    /// Options for sensitive information action.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The currently supported PII entities.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
