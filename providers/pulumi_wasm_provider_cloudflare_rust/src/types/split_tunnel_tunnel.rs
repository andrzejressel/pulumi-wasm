#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SplitTunnelTunnel {
    /// The address for the tunnel.
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// A description for the tunnel.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The domain name for the tunnel.
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
}
