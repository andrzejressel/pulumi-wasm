#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionConnectorVersionInfraConfig {
    /// (Output)
    /// Max QPS supported by the connector version before throttling of requests.
    #[builder(into, default)]
    #[serde(rename = "ratelimitThreshold")]
    pub r#ratelimit_threshold: Box<Option<String>>,
}
