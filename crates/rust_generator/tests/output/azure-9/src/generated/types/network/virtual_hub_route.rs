#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualHubRoute {
    /// A list of Address Prefixes.
    #[builder(into)]
    #[serde(rename = "addressPrefixes")]
    pub r#address_prefixes: Box<Vec<String>>,
    /// The IP Address that Packets should be forwarded to as the Next Hop.
    #[builder(into)]
    #[serde(rename = "nextHopIpAddress")]
    pub r#next_hop_ip_address: Box<String>,
}
