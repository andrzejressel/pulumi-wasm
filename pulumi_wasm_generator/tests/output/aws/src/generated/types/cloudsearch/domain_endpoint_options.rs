#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainEndpointOptions {
    /// Enables or disables the requirement that all requests to the domain arrive over HTTPS.
    #[builder(into, default)]
    #[serde(rename = "enforceHttps")]
    pub r#enforce_https: Box<Option<bool>>,
    /// The minimum required TLS version. See the [AWS documentation](https://docs.aws.amazon.com/cloudsearch/latest/developerguide/API_DomainEndpointOptions.html) for valid values.
    #[builder(into, default)]
    #[serde(rename = "tlsSecurityPolicy")]
    pub r#tls_security_policy: Box<Option<String>>,
}