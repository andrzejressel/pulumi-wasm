#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceTemplateSpecContainerStartupProbe {
    /// Minimum consecutive failures for the probe to be considered failed after
    /// having succeeded. Defaults to 3. Minimum value is 1.
    #[builder(into)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Box<i32>,
    /// GRPC specifies an action involving a GRPC port.
    #[builder(into)]
    #[serde(rename = "grpcs")]
    pub r#grpcs: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerStartupProbeGrpc>>,
    /// HttpGet specifies the http request to perform.
    #[builder(into)]
    #[serde(rename = "httpGets")]
    pub r#http_gets: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerStartupProbeHttpGet>>,
    /// Number of seconds after the container has started before the probe is
    /// initiated.
    /// Defaults to 0 seconds. Minimum value is 0. Maximum value is 240.
    #[builder(into)]
    #[serde(rename = "initialDelaySeconds")]
    pub r#initial_delay_seconds: Box<i32>,
    /// How often (in seconds) to perform the probe.
    /// Default to 10 seconds. Minimum value is 1. Maximum value is 240.
    #[builder(into)]
    #[serde(rename = "periodSeconds")]
    pub r#period_seconds: Box<i32>,
    /// TcpSocket specifies an action involving a TCP port.
    #[builder(into)]
    #[serde(rename = "tcpSockets")]
    pub r#tcp_sockets: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerStartupProbeTcpSocket>>,
    /// Number of seconds after which the probe times out.
    /// Defaults to 1 second. Minimum value is 1. Maximum value is 3600.
    /// Must be smaller than periodSeconds.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<i32>,
}
