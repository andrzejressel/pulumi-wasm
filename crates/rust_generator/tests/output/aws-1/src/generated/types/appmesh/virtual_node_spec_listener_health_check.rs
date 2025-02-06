#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecListenerHealthCheck {
    /// Number of consecutive successful health checks that must occur before declaring listener healthy.
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Box<i32>,
    /// Time period in milliseconds between each health check execution.
    #[builder(into)]
    #[serde(rename = "intervalMillis")]
    pub r#interval_millis: Box<i32>,
    /// Destination path for the health check request. This is only required if the specified protocol is `http` or `http2`.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Destination port for the health check request. This port must match the port defined in the `port_mapping` for the listener.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Protocol for the health check request. Valid values are `http`, `http2`, `tcp` and `grpc`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Amount of time to wait when receiving a response from the health check, in milliseconds.
    #[builder(into)]
    #[serde(rename = "timeoutMillis")]
    pub r#timeout_millis: Box<i32>,
    /// Number of consecutive failed health checks that must occur before declaring a virtual node unhealthy.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<i32>,
}
