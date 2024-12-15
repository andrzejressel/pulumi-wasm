#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FallbackDomainDomain {
    /// A description of the fallback domain, displayed in the client UI.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A list of IP addresses to handle domain resolution.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    /// The domain suffix to match when resolving locally.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
