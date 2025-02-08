#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceVectorIngestionConfigurationParsingConfigurationBedrockFoundationModelConfigurationParsingPrompt {
    /// Instructions for interpreting the contents of the document.
    #[builder(into)]
    #[serde(rename = "parsingPromptString")]
    pub r#parsing_prompt_string: Box<String>,
}
