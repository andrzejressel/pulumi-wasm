#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointConfigurationAsyncInferenceConfigClientConfig {
    /// The maximum number of concurrent requests sent by the SageMaker client to the model container. If no value is provided, Amazon SageMaker will choose an optimal value for you.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentInvocationsPerInstance")]
    pub r#max_concurrent_invocations_per_instance: Box<Option<i32>>,
}
