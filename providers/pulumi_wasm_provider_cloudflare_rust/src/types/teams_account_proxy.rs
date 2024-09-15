#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountProxy {
    /// Whether root ca is enabled account wide for ZT clients.
    #[builder(into)]
    #[serde(rename = "rootCa")]
    pub r#root_ca: Box<bool>,
    /// Whether gateway proxy is enabled on gateway devices for TCP traffic.
    #[builder(into)]
    #[serde(rename = "tcp")]
    pub r#tcp: Box<bool>,
    /// Whether gateway proxy is enabled on gateway devices for UDP traffic.
    #[builder(into)]
    #[serde(rename = "udp")]
    pub r#udp: Box<bool>,
}
