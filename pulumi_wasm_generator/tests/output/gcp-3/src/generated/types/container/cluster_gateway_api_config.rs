#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterGatewayApiConfig {
    /// Which Gateway Api channel should be used. `CHANNEL_DISABLED`, `CHANNEL_EXPERIMENTAL` or `CHANNEL_STANDARD`.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: Box<String>,
}
