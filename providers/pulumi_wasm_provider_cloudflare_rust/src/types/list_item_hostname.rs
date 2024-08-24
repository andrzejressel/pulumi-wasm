#[derive(serde::Serialize)]
pub struct ListItemHostname {
    /// The FQDN to match on.
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
