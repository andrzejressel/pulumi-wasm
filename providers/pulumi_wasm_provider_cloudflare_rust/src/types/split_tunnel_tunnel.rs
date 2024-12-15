#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SplitTunnelTunnel {
    /// The address for the tunnel.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// A description for the tunnel.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The domain name for the tunnel.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
}
