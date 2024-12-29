#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxEnvironmentCustomDnsConfiguration {
    /// IP address of the DNS server.
    #[builder(into)]
    #[serde(rename = "customDnsServerIp")]
    pub r#custom_dns_server_ip: Box<String>,
    /// Name of the DNS server.
    #[builder(into)]
    #[serde(rename = "customDnsServerName")]
    pub r#custom_dns_server_name: Box<String>,
}
