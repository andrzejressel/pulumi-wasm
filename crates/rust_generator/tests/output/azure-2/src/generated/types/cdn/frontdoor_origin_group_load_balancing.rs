#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorOriginGroupLoadBalancing {
    /// Specifies the additional latency in milliseconds for probes to fall into the lowest latency bucket. Possible values are between `0` and `1000` milliseconds (inclusive). Defaults to `50`.
    #[builder(into, default)]
    #[serde(rename = "additionalLatencyInMilliseconds")]
    pub r#additional_latency_in_milliseconds: Box<Option<i32>>,
    /// Specifies the number of samples to consider for load balancing decisions. Possible values are between `0` and `255` (inclusive). Defaults to `4`.
    #[builder(into, default)]
    #[serde(rename = "sampleSize")]
    pub r#sample_size: Box<Option<i32>>,
    /// Specifies the number of samples within the sample period that must succeed. Possible values are between `0` and `255` (inclusive). Defaults to `3`.
    #[builder(into, default)]
    #[serde(rename = "successfulSamplesRequired")]
    pub r#successful_samples_required: Box<Option<i32>>,
}
