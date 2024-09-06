#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers
    #[serde(rename = "nameservers")]
    pub r#nameservers: Box<Vec<String>>,
    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)
    #[serde(rename = "options")]
    pub r#options: Box<Option<Vec<String>>>,
    /// A search list for host-name lookup
    #[serde(rename = "searches")]
    pub r#searches: Box<Option<Vec<String>>>,
}
