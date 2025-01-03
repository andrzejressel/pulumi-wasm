#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TargetGroupConfigHealthCheck {
    /// Indicates whether health checking is enabled. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The approximate amount of time, in seconds, between health checks of an individual target. The range is 5–300 seconds. The default is 30 seconds.
    #[builder(into, default)]
    #[serde(rename = "healthCheckIntervalSeconds")]
    pub r#health_check_interval_seconds: Box<Option<i32>>,
    /// The amount of time, in seconds, to wait before reporting a target as unhealthy. The range is 1–120 seconds. The default is 5 seconds.
    /// * `healthy_threshold_count ` - (Optional) The number of consecutive successful health checks required before considering an unhealthy target healthy. The range is 2–10. The default is 5.
    #[builder(into, default)]
    #[serde(rename = "healthCheckTimeoutSeconds")]
    pub r#health_check_timeout_seconds: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "healthyThresholdCount")]
    pub r#healthy_threshold_count: Box<Option<i32>>,
    /// The codes to use when checking for a successful response from a target. These are called _Success codes_ in the console.
    #[builder(into, default)]
    #[serde(rename = "matcher")]
    pub r#matcher: Box<Option<super::super::types::vpclattice::TargetGroupConfigHealthCheckMatcher>>,
    /// The destination for health checks on the targets. If the protocol version is HTTP/1.1 or HTTP/2, specify a valid URI (for example, /path?query). The default path is `/`. Health checks are not supported if the protocol version is gRPC, however, you can choose HTTP/1.1 or HTTP/2 and specify a valid URI.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The port used when performing health checks on targets. The default setting is the port that a target receives traffic on.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// The protocol used when performing health checks on targets. The possible protocols are `HTTP` and `HTTPS`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// The protocol version used when performing health checks on targets. The possible protocol versions are `HTTP1` and `HTTP2`. The default is `HTTP1`.
    #[builder(into, default)]
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: Box<Option<String>>,
    /// The number of consecutive failed health checks required before considering a target unhealthy. The range is 2–10. The default is 2.
    #[builder(into, default)]
    #[serde(rename = "unhealthyThresholdCount")]
    pub r#unhealthy_threshold_count: Box<Option<i32>>,
}
