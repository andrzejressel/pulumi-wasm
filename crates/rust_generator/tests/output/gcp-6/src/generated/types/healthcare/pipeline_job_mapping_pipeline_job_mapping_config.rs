#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineJobMappingPipelineJobMappingConfig {
    /// Describes the mapping configuration.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Specifies the path to the mapping configuration for harmonization pipeline.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "whistleConfigSource")]
    pub r#whistle_config_source: Box<Option<super::super::types::healthcare::PipelineJobMappingPipelineJobMappingConfigWhistleConfigSource>>,
}
