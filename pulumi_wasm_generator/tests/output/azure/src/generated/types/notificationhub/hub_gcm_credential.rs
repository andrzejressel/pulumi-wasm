#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HubGcmCredential {
    /// The API Key associated with the Google Cloud Messaging service.
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Box<String>,
}