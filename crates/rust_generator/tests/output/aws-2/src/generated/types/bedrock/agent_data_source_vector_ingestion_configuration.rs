#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceVectorIngestionConfiguration {
    /// Details about how to chunk the documents in the data source. A chunk refers to an excerpt from a data source that is returned when the knowledge base that it belongs to is queried. See `chunking_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "chunkingConfiguration")]
    pub r#chunking_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfiguration>>,
    /// Configuration for custom transformation of data source documents.
    #[builder(into, default)]
    #[serde(rename = "customTransformationConfiguration")]
    pub r#custom_transformation_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfiguration>>,
    /// Configuration for custom parsing of data source documents. See `parsing_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "parsingConfiguration")]
    pub r#parsing_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationParsingConfiguration>>,
}
