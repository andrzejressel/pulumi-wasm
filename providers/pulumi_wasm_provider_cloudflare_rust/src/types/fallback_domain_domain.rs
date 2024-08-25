#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct FallbackDomainDomain {
    /// A description of the fallback domain, displayed in the client UI.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A list of IP addresses to handle domain resolution.
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    /// The domain suffix to match when resolving locally.
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
