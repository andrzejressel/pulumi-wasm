#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContainerNetworksAdvanced {
    /// The network aliases of the container in the specific network.
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    /// The IPV4 address of the container in the specific network.
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: Box<Option<String>>,
    /// The IPV6 address of the container in the specific network.
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
    /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
