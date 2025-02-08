#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ContainerServiceDeploymentVersionPublicEndpointHealthCheck {
    /// The number of consecutive health checks successes required before moving the container to the Healthy state. Defaults to 2.
    #[builder(into, default)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Box<Option<i32>>,
    /// The approximate interval, in seconds, between health checks of an individual container. You can specify between 5 and 300 seconds. Defaults to 5.
    #[builder(into, default)]
    #[serde(rename = "intervalSeconds")]
    pub r#interval_seconds: Box<Option<i32>>,
    /// The path on the container on which to perform the health check. Defaults to "/".
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The HTTP codes to use when checking for a successful response from a container. You can specify values between 200 and 499. Defaults to "200-499".
    #[builder(into, default)]
    #[serde(rename = "successCodes")]
    pub r#success_codes: Box<Option<String>>,
    /// The amount of time, in seconds, during which no response means a failed health check. You can specify between 2 and 60 seconds. Defaults to 2.
    #[builder(into, default)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<Option<i32>>,
    /// The number of consecutive health checks failures required before moving the container to the Unhealthy state. Defaults to 2.
    #[builder(into, default)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<Option<i32>>,
}
