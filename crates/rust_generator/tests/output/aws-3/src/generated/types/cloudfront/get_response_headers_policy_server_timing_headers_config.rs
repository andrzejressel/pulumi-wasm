#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResponseHeadersPolicyServerTimingHeadersConfig {
    /// Whether CloudFront adds the `Server-Timing` header to HTTP responses that it sends in response to requests that match a cache behavior that's associated with this response headers policy.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Number 0–100 (inclusive) that specifies the percentage of responses that you want CloudFront to add the Server-Timing header to.
    #[builder(into)]
    #[serde(rename = "samplingRate")]
    pub r#sampling_rate: Box<f64>,
}
