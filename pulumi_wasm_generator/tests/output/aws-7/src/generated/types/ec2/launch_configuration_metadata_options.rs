#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchConfigurationMetadataOptions {
    /// The state of the metadata service: `enabled`, `disabled`.
    #[builder(into, default)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Box<Option<String>>,
    /// The desired HTTP PUT response hop limit for instance metadata requests.
    #[builder(into, default)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Box<Option<i32>>,
    /// If session tokens are required: `optional`, `required`.
    #[builder(into, default)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Box<Option<String>>,
}
