#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointPrivateDnsZoneConfigRecordSet {
    /// The fully qualified domain name to the `private_dns_zone`.
    #[builder(into, default)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<Option<String>>,
    /// A list of all IP Addresses that map to the `private_dns_zone` fqdn.
    #[builder(into, default)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Option<Vec<String>>>,
    /// Specifies the Name of the Private Endpoint. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The time to live for each connection to the `private_dns_zone`.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
    /// The type of DNS record.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
