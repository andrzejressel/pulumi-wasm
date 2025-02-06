#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyFirewallPolicyStatefulEngineOptionsFlowTimeouts {
    /// Number of seconds that can pass without any TCP traffic sent through the firewall before the firewall determines that the connection is idle. After the idle timeout passes, data packets are dropped, however, the next TCP SYN packet is considered a new flow and is processed by the firewall. Clients or targets can use TCP keepalive packets to reset the idle timeout. Default value: `350`.
    #[builder(into, default)]
    #[serde(rename = "tcpIdleTimeoutSeconds")]
    pub r#tcp_idle_timeout_seconds: Box<Option<i32>>,
}
