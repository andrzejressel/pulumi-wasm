#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionAutomaticScalingRequestUtilization {
    /// Target number of concurrent requests.
    #[builder(into, default)]
    #[serde(rename = "targetConcurrentRequests")]
    pub r#target_concurrent_requests: Box<Option<f64>>,
    /// Target requests per second.
    #[builder(into, default)]
    #[serde(rename = "targetRequestCountPerSecond")]
    pub r#target_request_count_per_second: Box<Option<String>>,
}
