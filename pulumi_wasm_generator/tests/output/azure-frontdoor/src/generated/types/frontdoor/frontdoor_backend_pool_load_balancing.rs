#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorBackendPoolLoadBalancing {
    /// The additional latency in milliseconds for probes to fall into the lowest latency bucket. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "additionalLatencyMilliseconds")]
    pub r#additional_latency_milliseconds: Box<Option<i32>>,
    /// The ID of the FrontDoor.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the name of the Load Balancer.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The number of samples to consider for load balancing decisions. Defaults to `4`.
    #[builder(into, default)]
    #[serde(rename = "sampleSize")]
    pub r#sample_size: Box<Option<i32>>,
    /// The number of samples within the sample period that must succeed. Defaults to `2`.
    #[builder(into, default)]
    #[serde(rename = "successfulSamplesRequired")]
    pub r#successful_samples_required: Box<Option<i32>>,
}
