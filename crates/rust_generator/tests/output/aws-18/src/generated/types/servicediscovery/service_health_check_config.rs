#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceHealthCheckConfig {
    /// The number of consecutive health checks. Maximum value of 10.
    #[builder(into, default)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Box<Option<i32>>,
    /// The path that you want Route 53 to request when performing health checks. Route 53 automatically adds the DNS name for the service. If you don't specify a value, the default value is /.
    #[builder(into, default)]
    #[serde(rename = "resourcePath")]
    pub r#resource_path: Box<Option<String>>,
    /// The type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy. Valid Values: HTTP, HTTPS, TCP
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
