#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointConfigurationProductionVariantServerlessConfig {
    /// The maximum number of concurrent invocations your serverless endpoint can process. Valid values are between `1` and `200`.
    #[builder(into)]
    #[serde(rename = "maxConcurrency")]
    pub r#max_concurrency: Box<i32>,
    /// The memory size of your serverless endpoint. Valid values are in 1 GB increments: `1024` MB, `2048` MB, `3072` MB, `4096` MB, `5120` MB, or `6144` MB.
    #[builder(into)]
    #[serde(rename = "memorySizeInMb")]
    pub r#memory_size_in_mb: Box<i32>,
    /// The amount of provisioned concurrency to allocate for the serverless endpoint. Should be less than or equal to `max_concurrency`. Valid values are between `1` and `200`.
    #[builder(into, default)]
    #[serde(rename = "provisionedConcurrency")]
    pub r#provisioned_concurrency: Box<Option<i32>>,
}
