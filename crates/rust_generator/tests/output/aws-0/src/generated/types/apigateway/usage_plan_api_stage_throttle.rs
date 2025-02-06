#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UsagePlanApiStageThrottle {
    /// The API request burst limit, the maximum rate limit over a time ranging from one to a few seconds, depending upon whether the underlying token bucket is at its full capacity.
    #[builder(into, default)]
    #[serde(rename = "burstLimit")]
    pub r#burst_limit: Box<Option<i32>>,
    /// Method to apply the throttle settings for. Specfiy the path and method, for example `/test/GET`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The API request steady-state rate limit.
    #[builder(into, default)]
    #[serde(rename = "rateLimit")]
    pub r#rate_limit: Box<Option<f64>>,
}
