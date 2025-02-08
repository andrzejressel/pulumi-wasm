#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfiguration {
    /// Maximum number of tokens to include in a chunk. Must contain two `level_configurations`. See `level_configurations` for details.
    #[builder(into)]
    #[serde(rename = "levelConfigurations")]
    pub r#level_configurations: Box<Vec<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfigurationLevelConfiguration>>,
    /// The number of tokens to repeat across chunks in the same layer.
    #[builder(into)]
    #[serde(rename = "overlapTokens")]
    pub r#overlap_tokens: Box<f64>,
}
