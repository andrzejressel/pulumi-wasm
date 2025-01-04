#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProfilingGroupAgentOrchestrationConfig {
    /// (Required) Boolean that specifies whether the profiling agent collects profiling data or
    #[builder(into)]
    #[serde(rename = "profilingEnabled")]
    pub r#profiling_enabled: Box<bool>,
}
