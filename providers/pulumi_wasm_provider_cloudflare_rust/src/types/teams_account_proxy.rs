#[derive(serde::Serialize)]
pub struct TeamsAccountProxy {
    /// Whether root ca is enabled account wide for ZT clients.
    #[serde(rename = "rootCa")]
    pub r#root_ca: Box<bool>,
    /// Whether gateway proxy is enabled on gateway devices for TCP traffic.
    #[serde(rename = "tcp")]
    pub r#tcp: Box<bool>,
    /// Whether gateway proxy is enabled on gateway devices for UDP traffic.
    #[serde(rename = "udp")]
    pub r#udp: Box<bool>,
}
