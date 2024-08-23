#[derive(serde::Serialize)]
pub struct FallbackDomainDomain {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
