#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentAgentPromptOverrideConfigurationPromptConfigurationInferenceConfiguration {
    /// Maximum number of tokens to allow in the generated response.
    #[builder(into)]
    #[serde(rename = "maxLength")]
    pub r#max_length: Box<i32>,
    /// List of stop sequences. A stop sequence is a sequence of characters that causes the model to stop generating the response.
    #[builder(into)]
    #[serde(rename = "stopSequences")]
    pub r#stop_sequences: Box<Vec<String>>,
    /// Likelihood of the model selecting higher-probability options while generating a response. A lower value makes the model more likely to choose higher-probability options, while a higher value makes the model more likely to choose lower-probability options.
    #[builder(into)]
    #[serde(rename = "temperature")]
    pub r#temperature: Box<f64>,
    /// Number of top most-likely candidates, between 0 and 500, from which the model chooses the next token in the sequence.
    #[builder(into)]
    #[serde(rename = "topK")]
    pub r#top_k: Box<i32>,
    /// Top percentage of the probability distribution of next tokens, between 0 and 1 (denoting 0% and 100%), from which the model chooses the next token in the sequence.
    #[builder(into)]
    #[serde(rename = "topP")]
    pub r#top_p: Box<f64>,
}
