#[derive(serde::Serialize)]
pub struct TeamsAccountFips {
    /// Only allow FIPS-compliant TLS configuration.
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<bool>>,
}
