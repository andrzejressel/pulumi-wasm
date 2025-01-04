#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetNetworkProfileDnsSettings {
    /// Specifies an array of DNS servers.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Vec<String>>,
}
