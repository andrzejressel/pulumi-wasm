#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ListItemValueHostname {
    /// The FQDN to match on. Wildcard sub-domain matching is allowed. Eg. *.abc.com.
    #[builder(into)]
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
