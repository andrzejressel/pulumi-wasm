#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustLocalFallbackDomainDomain {
    /// A description of the fallback domain, displayed in the client UI.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A list of IP addresses to handle domain resolution.
    #[builder(into, default)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    /// The domain suffix to match when resolving locally.
    #[builder(into, default)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
