#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateSpecContainerLivenessProbe {
    /// Minimum consecutive failures for the probe to be considered failed after
    /// having succeeded. Defaults to 3. Minimum value is 1.
    #[builder(into, default)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Box<Option<i32>>,
    /// GRPC specifies an action involving a GRPC port.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "grpc")]
    pub r#grpc: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerLivenessProbeGrpc>>,
    /// HttpGet specifies the http request to perform.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "httpGet")]
    pub r#http_get: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerLivenessProbeHttpGet>>,
    /// Number of seconds after the container has started before the probe is
    /// initiated.
    /// Defaults to 0 seconds. Minimum value is 0. Maximum value is 3600.
    #[builder(into, default)]
    #[serde(rename = "initialDelaySeconds")]
    pub r#initial_delay_seconds: Box<Option<i32>>,
    /// How often (in seconds) to perform the probe.
    /// Default to 10 seconds. Minimum value is 1. Maximum value is 3600.
    #[builder(into, default)]
    #[serde(rename = "periodSeconds")]
    pub r#period_seconds: Box<Option<i32>>,
    /// Number of seconds after which the probe times out.
    /// Defaults to 1 second. Minimum value is 1. Maximum value is 3600.
    /// Must be smaller than period_seconds.
    #[builder(into, default)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<Option<i32>>,
}
