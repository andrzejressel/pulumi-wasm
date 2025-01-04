#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterHttpProxyConfig {
    /// The proxy address to be used when communicating over HTTP.
    #[builder(into, default)]
    #[serde(rename = "httpProxy")]
    pub r#http_proxy: Box<Option<String>>,
    /// The proxy address to be used when communicating over HTTPS.
    #[builder(into, default)]
    #[serde(rename = "httpsProxy")]
    pub r#https_proxy: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "noProxies")]
    pub r#no_proxies: Box<Option<Vec<String>>>,
    /// The base64 encoded alternative CA certificate content in PEM format.
    #[builder(into, default)]
    #[serde(rename = "trustedCa")]
    pub r#trusted_ca: Box<Option<String>>,
}
