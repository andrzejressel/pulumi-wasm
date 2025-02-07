#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualHubConnectionRoutingStaticVnetRoute {
    /// A list of CIDR Ranges which should be used as Address Prefixes.
    #[builder(into, default)]
    #[serde(rename = "addressPrefixes")]
    pub r#address_prefixes: Box<Option<Vec<String>>>,
    /// The name which should be used for this Static Route.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The IP Address which should be used for the Next Hop.
    #[builder(into, default)]
    #[serde(rename = "nextHopIpAddress")]
    pub r#next_hop_ip_address: Box<Option<String>>,
}
