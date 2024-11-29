#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ListItemHostname {
    /// The FQDN to match on.
    #[builder(into)]
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
