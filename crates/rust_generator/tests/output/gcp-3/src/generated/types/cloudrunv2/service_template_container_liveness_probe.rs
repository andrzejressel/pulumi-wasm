#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceTemplateContainerLivenessProbe {
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[builder(into, default)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Box<Option<i32>>,
    /// GRPC specifies an action involving a GRPC port.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "grpc")]
    pub r#grpc: Box<Option<super::super::types::cloudrunv2::ServiceTemplateContainerLivenessProbeGrpc>>,
    /// HTTPGet specifies the http request to perform.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "httpGet")]
    pub r#http_get: Box<Option<super::super::types::cloudrunv2::ServiceTemplateContainerLivenessProbeHttpGet>>,
    /// Number of seconds after the container has started before the probe is initiated. Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[builder(into, default)]
    #[serde(rename = "initialDelaySeconds")]
    pub r#initial_delay_seconds: Box<Option<i32>>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe is 3600. Maximum value for startup probe is 240. Must be greater or equal than timeoutSeconds
    #[builder(into, default)]
    #[serde(rename = "periodSeconds")]
    pub r#period_seconds: Box<Option<i32>>,
    /// TCPSocketAction describes an action based on opening a socket
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tcpSocket")]
    pub r#tcp_socket: Box<Option<super::super::types::cloudrunv2::ServiceTemplateContainerLivenessProbeTcpSocket>>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. Maximum value is 3600. Must be smaller than periodSeconds. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[builder(into, default)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<Option<i32>>,
}
