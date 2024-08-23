#[derive(serde::Serialize)]
pub struct ContainerNetworkData {
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    #[serde(rename = "globalIpv6Address")]
    pub r#global_ipv_6_address: Box<Option<String>>,
    #[serde(rename = "globalIpv6PrefixLength")]
    pub r#global_ipv_6_prefix_length: Box<Option<i32>>,
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    #[serde(rename = "ipPrefixLength")]
    pub r#ip_prefix_length: Box<Option<i32>>,
    #[serde(rename = "ipv6Gateway")]
    pub r#ipv_6_gateway: Box<Option<String>>,
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    #[serde(rename = "networkName")]
    pub r#network_name: Box<Option<String>>,
}
