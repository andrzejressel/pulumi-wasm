#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TlsRouteRuleMatch {
    /// ALPN (Application-Layer Protocol Negotiation) to match against. Examples: "http/1.1", "h2". At least one of sniHost and alpn is required. Up to 5 alpns across all matches can be set.
    #[builder(into, default)]
    #[serde(rename = "alpns")]
    pub r#alpns: Box<Option<Vec<String>>>,
    /// SNI (server name indicator) to match against. SNI will be matched against all wildcard domains, i.e. www.example.com will be first matched against www.example.com, then *.example.com, then *.com.
    /// Partial wildcards are not supported, and values like *w.example.com are invalid. At least one of sniHost and alpn is required. Up to 5 sni hosts across all matches can be set.
    #[builder(into, default)]
    #[serde(rename = "sniHosts")]
    pub r#sni_hosts: Box<Option<Vec<String>>>,
}
