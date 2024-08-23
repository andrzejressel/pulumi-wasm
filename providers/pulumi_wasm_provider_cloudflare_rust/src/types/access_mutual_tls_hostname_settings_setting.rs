#[derive(serde::Serialize)]
pub struct AccessMutualTlsHostnameSettingsSetting {
    #[serde(rename = "chinaNetwork")]
    pub r#china_network: Box<Option<bool>>,
    #[serde(rename = "clientCertificateForwarding")]
    pub r#client_certificate_forwarding: Box<Option<bool>>,
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
}
