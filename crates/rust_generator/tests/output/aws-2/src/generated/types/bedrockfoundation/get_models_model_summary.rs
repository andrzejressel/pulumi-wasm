#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetModelsModelSummary {
    /// Customizations that the model supports.
    #[builder(into)]
    #[serde(rename = "customizationsSupporteds")]
    pub r#customizations_supporteds: Box<Vec<String>>,
    /// Inference types that the model supports.
    #[builder(into)]
    #[serde(rename = "inferenceTypesSupporteds")]
    pub r#inference_types_supporteds: Box<Vec<String>>,
    /// Input modalities that the model supports.
    #[builder(into)]
    #[serde(rename = "inputModalities")]
    pub r#input_modalities: Box<Vec<String>>,
    /// Model ARN.
    #[builder(into)]
    #[serde(rename = "modelArn")]
    pub r#model_arn: Box<String>,
    /// Model identifier.
    #[builder(into)]
    #[serde(rename = "modelId")]
    pub r#model_id: Box<String>,
    /// Model name.
    #[builder(into)]
    #[serde(rename = "modelName")]
    pub r#model_name: Box<String>,
    /// Output modalities that the model supports.
    #[builder(into)]
    #[serde(rename = "outputModalities")]
    pub r#output_modalities: Box<Vec<String>>,
    /// Model provider name.
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Box<String>,
    /// Indicates whether the model supports streaming.
    #[builder(into)]
    #[serde(rename = "responseStreamingSupported")]
    pub r#response_streaming_supported: Box<bool>,
}
