#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentAgentPromptOverrideConfigurationPromptConfiguration {
    /// prompt template with which to replace the default prompt template. You can use placeholder variables in the base prompt template to customize the prompt. For more information, see [Prompt template placeholder variables](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-placeholders.html).
    #[builder(into)]
    #[serde(rename = "basePromptTemplate")]
    pub r#base_prompt_template: Box<String>,
    /// Inference parameters to use when the agent invokes a foundation model in the part of the agent sequence defined by the `prompt_type`. For more information, see [Inference parameters for foundation models](https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html). See `inference_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "inferenceConfigurations")]
    pub r#inference_configurations: Box<Vec<super::super::types::bedrock::AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration>>,
    /// Whether to override the default parser Lambda function when parsing the raw foundation model output in the part of the agent sequence defined by the `prompt_type`. If you set the argument as `OVERRIDDEN`, the `override_lambda` argument in the `prompt_override_configuration` block must be specified with the ARN of a Lambda function. Valid values: `DEFAULT`, `OVERRIDDEN`.
    #[builder(into)]
    #[serde(rename = "parserMode")]
    pub r#parser_mode: Box<String>,
    /// Whether to override the default prompt template for this `prompt_type`. Set this argument to `OVERRIDDEN` to use the prompt that you provide in the `base_prompt_template`. If you leave it as `DEFAULT`, the agent uses a default prompt template. Valid values: `DEFAULT`, `OVERRIDDEN`.
    #[builder(into)]
    #[serde(rename = "promptCreationMode")]
    pub r#prompt_creation_mode: Box<String>,
    /// Whether to allow the agent to carry out the step specified in the `prompt_type`. If you set this argument to `DISABLED`, the agent skips that step. Valid Values: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "promptState")]
    pub r#prompt_state: Box<String>,
    /// Step in the agent sequence that this prompt configuration applies to. Valid values: `PRE_PROCESSING`, `ORCHESTRATION`, `POST_PROCESSING`, `KNOWLEDGE_BASE_RESPONSE_GENERATION`.
    #[builder(into)]
    #[serde(rename = "promptType")]
    pub r#prompt_type: Box<String>,
}
