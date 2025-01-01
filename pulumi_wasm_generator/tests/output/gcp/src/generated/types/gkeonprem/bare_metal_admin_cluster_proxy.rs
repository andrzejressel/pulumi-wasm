#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterProxy {
    /// A list of IPs, hostnames, and domains that should skip the proxy.
    /// For example: ["127.0.0.1", "example.com", ".corp", "localhost"].
    #[builder(into, default)]
    #[serde(rename = "noProxies")]
    pub r#no_proxies: Box<Option<Vec<String>>>,
    /// Specifies the address of your proxy server.
    /// For Example: http://domain
    /// WARNING: Do not provide credentials in the format
    /// of http://(username:password@)domain these will be rejected by the server.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
