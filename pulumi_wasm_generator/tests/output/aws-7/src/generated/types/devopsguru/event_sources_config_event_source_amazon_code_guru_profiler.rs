#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSourcesConfigEventSourceAmazonCodeGuruProfiler {
    /// Status of the CodeGuru Profiler integration. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
