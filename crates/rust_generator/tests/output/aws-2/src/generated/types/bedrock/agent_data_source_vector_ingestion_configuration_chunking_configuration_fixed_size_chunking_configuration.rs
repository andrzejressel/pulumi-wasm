#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfigurationFixedSizeChunkingConfiguration {
    /// Maximum number of tokens to include in a chunk.
    #[builder(into)]
    #[serde(rename = "maxTokens")]
    pub r#max_tokens: Box<i32>,
    /// Percentage of overlap between adjacent chunks of a data source.
    #[builder(into)]
    #[serde(rename = "overlapPercentage")]
    pub r#overlap_percentage: Box<i32>,
}
