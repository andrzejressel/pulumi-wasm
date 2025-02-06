#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineBufferOptions {
    /// Whether persistent buffering should be enabled.
    #[builder(into)]
    #[serde(rename = "persistentBufferEnabled")]
    pub r#persistent_buffer_enabled: Box<bool>,
}
