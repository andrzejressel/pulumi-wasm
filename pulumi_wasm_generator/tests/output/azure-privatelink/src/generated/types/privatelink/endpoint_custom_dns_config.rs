#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointCustomDnsConfig {
    /// The fully qualified domain name to the `private_dns_zone`.
    #[builder(into, default)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<Option<String>>,
    /// A list of all IP Addresses that map to the `private_dns_zone` fqdn.
    #[builder(into, default)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Option<Vec<String>>>,
}