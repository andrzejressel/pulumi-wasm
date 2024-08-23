#[derive(serde::Serialize)]
pub struct DeviceManagedNetworksConfig {
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<String>,
    #[serde(rename = "tlsSockaddr")]
    pub r#tls_sockaddr: Box<String>,
}
