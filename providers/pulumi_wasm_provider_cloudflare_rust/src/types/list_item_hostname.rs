#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ListItemHostname {
    /// The FQDN to match on.
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
