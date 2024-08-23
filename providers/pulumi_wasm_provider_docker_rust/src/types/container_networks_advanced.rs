#[derive(serde::Serialize)]
pub struct ContainerNetworksAdvanced {
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: Box<Option<String>>,
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
