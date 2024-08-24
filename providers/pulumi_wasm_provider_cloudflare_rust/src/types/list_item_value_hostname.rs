#[derive(serde::Serialize)]
pub struct ListItemValueHostname {
    /// The FQDN to match on. Wildcard sub-domain matching is allowed. Eg. *.abc.com.
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
