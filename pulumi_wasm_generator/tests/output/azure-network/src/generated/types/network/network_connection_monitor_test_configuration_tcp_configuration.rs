#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkConnectionMonitorTestConfigurationTcpConfiguration {
    /// The destination port behavior for the TCP connection. Possible values are `None` and `ListenIfAvailable`.
    #[builder(into, default)]
    #[serde(rename = "destinationPortBehavior")]
    pub r#destination_port_behavior: Box<Option<String>>,
    /// The port for the TCP connection.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Should path evaluation with trace route be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "traceRouteEnabled")]
    pub r#trace_route_enabled: Box<Option<bool>>,
}