#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountThrottleSetting {
    /// Absolute maximum number of times API Gateway allows the API to be called per second (RPS).
    #[builder(into)]
    #[serde(rename = "burstLimit")]
    pub r#burst_limit: Box<f64>,
    /// Number of times API Gateway allows the API to be called per second on average (RPS).
    #[builder(into)]
    #[serde(rename = "rateLimit")]
    pub r#rate_limit: Box<f64>,
}
