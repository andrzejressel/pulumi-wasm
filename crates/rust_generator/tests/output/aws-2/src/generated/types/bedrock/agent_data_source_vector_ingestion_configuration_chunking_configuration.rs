#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfiguration {
    /// Option for chunking your source data, either in fixed-sized chunks or as one chunk. Valid values: `FIXED_SIZE`, `HIERARCHICAL`, `SEMANTIC`, `NONE`.
    #[builder(into)]
    #[serde(rename = "chunkingStrategy")]
    pub r#chunking_strategy: Box<String>,
    /// Configurations for when you choose fixed-size chunking. Requires chunking_strategy as `FIXED_SIZE`. See `fixed_size_chunking_configuration` for details.
    #[builder(into, default)]
    #[serde(rename = "fixedSizeChunkingConfiguration")]
    pub r#fixed_size_chunking_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationFixedSizeChunkingConfiguration>>,
    /// Configurations for when you choose hierarchical chunking. Requires chunking_strategy as `HIERARCHICAL`. See `hierarchical_chunking_configuration` for details.
    #[builder(into, default)]
    #[serde(rename = "hierarchicalChunkingConfiguration")]
    pub r#hierarchical_chunking_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfiguration>>,
    /// Configurations for when you choose semantic chunking. Requires chunking_strategy as `SEMANTIC`. See `semantic_chunking_configuration` for details.
    #[builder(into, default)]
    #[serde(rename = "semanticChunkingConfiguration")]
    pub r#semantic_chunking_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationSemanticChunkingConfiguration>>,
}
