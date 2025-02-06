#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciLogicalNetworkSubnetRoute {
    /// The Address in CIDR notation. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: Box<String>,
    /// The name of the route. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The IPv4 address of the next hop. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "nextHopIpAddress")]
    pub r#next_hop_ip_address: Box<String>,
}
