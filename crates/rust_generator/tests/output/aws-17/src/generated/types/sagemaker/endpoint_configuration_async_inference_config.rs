#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointConfigurationAsyncInferenceConfig {
    /// Configures the behavior of the client used by Amazon SageMaker to interact with the model container during asynchronous inference.
    #[builder(into, default)]
    #[serde(rename = "clientConfig")]
    pub r#client_config: Box<Option<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigClientConfig>>,
    /// Specifies the configuration for asynchronous inference invocation outputs.
    #[builder(into)]
    #[serde(rename = "outputConfig")]
    pub r#output_config: Box<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigOutputConfig>,
}
