#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipelineBufferOptions {
    /// Whether persistent buffering should be enabled.
    #[builder(into)]
    #[serde(rename = "persistentBufferEnabled")]
    pub r#persistent_buffer_enabled: Box<bool>,
}
