#[derive(serde::Serialize)]
pub struct CustomHostnameSslSetting {
    #[serde(rename = "ciphers")]
    pub r#ciphers: Box<Option<Vec<String>>>,
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Box<Option<String>>,
    #[serde(rename = "http2")]
    pub r#http_2: Box<Option<String>>,
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    #[serde(rename = "tls13")]
    pub r#tls_13: Box<Option<String>>,
}
