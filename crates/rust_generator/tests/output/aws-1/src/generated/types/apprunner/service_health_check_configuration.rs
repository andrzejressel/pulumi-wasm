#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceHealthCheckConfiguration {
    /// Number of consecutive checks that must succeed before App Runner decides that the service is healthy. Defaults to 1. Minimum value of 1. Maximum value of 20.
    #[builder(into, default)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Box<Option<i32>>,
    /// Time interval, in seconds, between health checks. Defaults to 5. Minimum value of 1. Maximum value of 20.
    #[builder(into, default)]
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<i32>>,
    /// URL to send requests to for health checks. Defaults to `/`. Minimum length of 0. Maximum length of 51200.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// IP protocol that App Runner uses to perform health checks for your service. Valid values: `TCP`, `HTTP`. Defaults to `TCP`. If you set protocol to `HTTP`, App Runner sends health check requests to the HTTP path specified by `path`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// Time, in seconds, to wait for a health check response before deciding it failed. Defaults to 2. Minimum value of  1. Maximum value of 20.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
    /// Number of consecutive checks that must fail before App Runner decides that the service is unhealthy. Defaults to 5. Minimum value of  1. Maximum value of 20.
    #[builder(into, default)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<Option<i32>>,
}
