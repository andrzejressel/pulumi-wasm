#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountProxy {
    /// Sets the time limit in seconds that a user can use an override code to bypass WARP.
    #[builder(into)]
    #[serde(rename = "disableForTime")]
    pub r#disable_for_time: Box<i32>,
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
    /// Whether virtual IP (CGNAT) is enabled account wide and will override existing local interface IP for ZT clients.
    #[builder(into)]
    #[serde(rename = "virtualIp")]
    pub r#virtual_ip: Box<bool>,
}
