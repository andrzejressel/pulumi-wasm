#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInfrastructureConfigurationInstanceMetadataOption {
    /// Number of hops that an instance can traverse to reach its destonation.
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Box<i32>,
    /// Whether a signed token is required for instance metadata retrieval requests.
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Box<String>,
}