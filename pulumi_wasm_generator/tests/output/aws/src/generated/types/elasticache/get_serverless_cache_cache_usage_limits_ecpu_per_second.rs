#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServerlessCacheCacheUsageLimitsEcpuPerSecond {
    /// The maximum number of ECPUs the cache can consume per second.
    #[builder(into)]
    #[serde(rename = "maximum")]
    pub r#maximum: Box<i32>,
    /// The minimum number of ECPUs the cache can consume per second.
    #[builder(into)]
    #[serde(rename = "minimum")]
    pub r#minimum: Box<i32>,
}