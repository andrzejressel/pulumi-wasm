#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceVectorIngestionConfigurationCustomTransformationConfiguration {
    /// The intermediate storage for custom transformation.
    #[builder(into, default)]
    #[serde(rename = "intermediateStorage")]
    pub r#intermediate_storage: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationIntermediateStorage>>,
    /// A custom processing step for documents moving through the data source ingestion pipeline.
    #[builder(into, default)]
    #[serde(rename = "transformation")]
    pub r#transformation: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformation>>,
}
