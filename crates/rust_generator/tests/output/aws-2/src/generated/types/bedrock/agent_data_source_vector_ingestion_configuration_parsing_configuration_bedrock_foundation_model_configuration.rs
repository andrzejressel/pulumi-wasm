#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfiguration {
    /// The ARN of the model used to parse documents
    #[builder(into)]
    #[serde(rename = "modelArn")]
    pub r#model_arn: Box<String>,
    /// Instructions for interpreting the contents of the document. See `parsing_prompt` block for details.
    #[builder(into, default)]
    #[serde(rename = "parsingPrompt")]
    pub r#parsing_prompt: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfigurationParsingPrompt>>,
}
