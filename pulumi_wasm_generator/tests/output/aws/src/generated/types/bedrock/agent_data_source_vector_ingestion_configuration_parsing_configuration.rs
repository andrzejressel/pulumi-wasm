#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceVectorIngestionConfigurationParsingConfiguration {
    /// Settings for a foundation model used to parse documents in a data source. See `bedrock_foundation_model_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "bedrockFoundationModelConfiguration")]
    pub r#bedrock_foundation_model_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfiguration>>,
    /// Currently only `BEDROCK_FOUNDATION_MODEL` is supported
    #[builder(into)]
    #[serde(rename = "parsingStrategy")]
    pub r#parsing_strategy: Box<String>,
}
