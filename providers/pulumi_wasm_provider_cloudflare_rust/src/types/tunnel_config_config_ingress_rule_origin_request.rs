#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequest {
    #[serde(rename = "access")]
    pub r#access: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequestAccess>>,
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Box<Option<bool>>,
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<Option<String>>,
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Box<Option<String>>,
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Box<Option<bool>>,
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Box<Option<bool>>,
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Box<Option<String>>,
    #[serde(rename = "ipRules")]
    pub r#ip_rules:
        Box<Option<Vec<crate::types::TunnelConfigConfigIngressRuleOriginRequestIpRule>>>,
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Box<Option<i32>>,
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Box<Option<String>>,
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Box<Option<bool>>,
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Box<Option<bool>>,
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Box<Option<String>>,
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Box<Option<String>>,
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Box<Option<i32>>,
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Box<Option<String>>,
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Box<Option<String>>,
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Box<Option<String>>,
}