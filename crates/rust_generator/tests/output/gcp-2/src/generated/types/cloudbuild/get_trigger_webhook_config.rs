#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerWebhookConfig {
    /// Resource name for the secret required as a URL parameter.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<String>,
    /// Potential issues with the underlying Pub/Sub subscription configuration.
    /// Only populated on get requests.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
