#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualHubConnectionRoutingStaticVnetRoute {
    /// A list of CIDR Ranges which is used as Address Prefixes.
    #[builder(into)]
    #[serde(rename = "addressPrefixes")]
    pub r#address_prefixes: Box<Vec<String>>,
    /// The name of the Connection which should be retrieved.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The IP Address which is used for the Next Hop.
    #[builder(into)]
    #[serde(rename = "nextHopIpAddress")]
    pub r#next_hop_ip_address: Box<String>,
}