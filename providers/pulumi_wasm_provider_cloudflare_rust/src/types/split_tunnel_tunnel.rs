#[derive(serde::Serialize)]
pub struct SplitTunnelTunnel {
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
}
