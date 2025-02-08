#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UsagePlanThrottleSettings {
    #[builder(into, default)]
    #[serde(rename = "burstLimit")]
    pub r#burst_limit: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "rateLimit")]
    pub r#rate_limit: Box<Option<f64>>,
}
