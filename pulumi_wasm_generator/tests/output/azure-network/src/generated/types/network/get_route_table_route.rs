#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteTableRoute {
    /// The destination CIDR to which the route applies.
    #[builder(into)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: Box<String>,
    /// The name of the Route Table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Contains the IP address packets should be forwarded to.
    #[builder(into)]
    #[serde(rename = "nextHopInIpAddress")]
    pub r#next_hop_in_ip_address: Box<String>,
    /// The type of Azure hop the packet should be sent to.
    #[builder(into)]
    #[serde(rename = "nextHopType")]
    pub r#next_hop_type: Box<String>,
}
