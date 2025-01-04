#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobEventTriggerConfigScaleRuleAuthentication {
    /// Name of the secret from which to pull the auth params.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
    /// Trigger Parameter that uses the secret.
    #[builder(into)]
    #[serde(rename = "triggerParameter")]
    pub r#trigger_parameter: Box<String>,
}
