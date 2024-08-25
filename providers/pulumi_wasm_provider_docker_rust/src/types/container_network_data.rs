#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContainerNetworkData {
    /// The network gateway of the container.
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The IPV6 address of the container.
    #[serde(rename = "globalIpv6Address")]
    pub r#global_ipv_6_address: Box<Option<String>>,
    /// The IPV6 prefix length address of the container.
    #[serde(rename = "globalIpv6PrefixLength")]
    pub r#global_ipv_6_prefix_length: Box<Option<i32>>,
    /// The IP address of the container.
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The IP prefix length of the container.
    #[serde(rename = "ipPrefixLength")]
    pub r#ip_prefix_length: Box<Option<i32>>,
    /// The IPV6 gateway of the container.
    #[serde(rename = "ipv6Gateway")]
    pub r#ipv_6_gateway: Box<Option<String>>,
    /// The MAC address of the container.
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    /// The name of the network
    #[serde(rename = "networkName")]
    pub r#network_name: Box<Option<String>>,
}
