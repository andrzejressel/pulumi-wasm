#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NextGenerationFirewallVirtualNetworkLocalRulestackDestinationNatBackendConfig {
    /// The port number to send traffic to.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The IP Address to send the traffic to.
    #[builder(into)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: Box<String>,
}