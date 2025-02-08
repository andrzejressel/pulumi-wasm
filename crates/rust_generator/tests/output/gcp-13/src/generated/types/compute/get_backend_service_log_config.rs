#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBackendServiceLogConfig {
    /// Whether to enable logging for the load balancer traffic served by this backend service.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Box<bool>,
    /// This field can only be specified if logging is enabled for this backend service. The value of
    /// the field must be in [0, 1]. This configures the sampling rate of requests to the load balancer
    /// where 1.0 means all logged requests are reported and 0.0 means no logged requests are reported.
    /// The default value is 1.0.
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<f64>,
}
