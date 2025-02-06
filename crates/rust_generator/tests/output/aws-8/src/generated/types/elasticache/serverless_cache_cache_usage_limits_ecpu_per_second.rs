#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServerlessCacheCacheUsageLimitsEcpuPerSecond {
    /// The maximum number of ECPUs the cache can consume per second. Must be between 1,000 and 15,000,000.
    #[builder(into, default)]
    #[serde(rename = "maximum")]
    pub r#maximum: Box<Option<i32>>,
    /// The minimum number of ECPUs the cache can consume per second. Must be between 1,000 and 15,000,000.
    #[builder(into, default)]
    #[serde(rename = "minimum")]
    pub r#minimum: Box<Option<i32>>,
}
