#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeSourceParametersActivemqBrokerParameters {
    /// The maximum number of records to include in each batch. Maximum value of 10000.
    #[builder(into, default)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Box<Option<i32>>,
    /// The credentials needed to access the resource. Detailed below.
    #[builder(into)]
    #[serde(rename = "credentials")]
    pub r#credentials: Box<super::super::types::pipes::PipeSourceParametersActivemqBrokerParametersCredentials>,
    /// The maximum length of a time to wait for events. Maximum value of 300.
    #[builder(into, default)]
    #[serde(rename = "maximumBatchingWindowInSeconds")]
    pub r#maximum_batching_window_in_seconds: Box<Option<i32>>,
    /// The name of the destination queue to consume. Maximum length of 1000.
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: Box<String>,
}
