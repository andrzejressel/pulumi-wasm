#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TunnelConfigConfigIngressRuleOriginRequest {
    /// Access rules for the ingress service.
    #[builder(into, default)]
    #[serde(rename = "access")]
    pub r#access: Box<Option<super::types::TunnelConfigConfigIngressRuleOriginRequestAccess>>,
    /// Runs as jump host.
    #[builder(into, default)]
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Box<Option<bool>>,
    /// Path to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `""`.
    #[builder(into, default)]
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<Option<String>>,
    /// Timeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`.
    #[builder(into, default)]
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Box<Option<String>>,
    /// Disables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Box<Option<bool>>,
    /// Enables HTTP/2 support for the origin connection. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Box<Option<bool>>,
    /// Sets the HTTP Host header on requests sent to the local service. Defaults to `""`.
    #[builder(into, default)]
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Box<Option<String>>,
    /// IP rules for the proxy service.
    #[builder(into, default)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<super::types::TunnelConfigConfigIngressRuleOriginRequestIpRule>>>,
    /// Maximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`.
    #[builder(into, default)]
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Box<Option<i32>>,
    /// Timeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`.
    #[builder(into, default)]
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Box<Option<String>>,
    /// Disable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Box<Option<bool>>,
    /// Disables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Box<Option<bool>>,
    /// Hostname that cloudflared should expect from your origin server certificate. Defaults to `""`.
    #[builder(into, default)]
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Box<Option<String>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`.
    #[builder(into, default)]
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Box<Option<String>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Box<Option<i32>>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `""`, `socks`. Defaults to `""`.
    #[builder(into, default)]
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Box<Option<String>>,
    /// The timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`.
    #[builder(into, default)]
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Box<Option<String>>,
    /// Timeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`.
    #[builder(into, default)]
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Box<Option<String>>,
}
