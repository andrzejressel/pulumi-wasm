#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers
    #[builder(into)]
    #[serde(rename = "nameservers")]
    pub r#nameservers: Box<Vec<String>>,
    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<Vec<String>>>,
    /// A search list for host-name lookup
    #[builder(into, default)]
    #[serde(rename = "searches")]
    pub r#searches: Box<Option<Vec<String>>>,
}
