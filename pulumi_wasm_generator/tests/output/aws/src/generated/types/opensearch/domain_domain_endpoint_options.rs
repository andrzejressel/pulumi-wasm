#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDomainEndpointOptions {
    /// Fully qualified domain for your custom endpoint.
    #[builder(into, default)]
    #[serde(rename = "customEndpoint")]
    pub r#custom_endpoint: Box<Option<String>>,
    /// ACM certificate ARN for your custom endpoint.
    #[builder(into, default)]
    #[serde(rename = "customEndpointCertificateArn")]
    pub r#custom_endpoint_certificate_arn: Box<Option<String>>,
    /// Whether to enable custom endpoint for the OpenSearch domain.
    #[builder(into, default)]
    #[serde(rename = "customEndpointEnabled")]
    pub r#custom_endpoint_enabled: Box<Option<bool>>,
    /// Whether or not to require HTTPS. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enforceHttps")]
    pub r#enforce_https: Box<Option<bool>>,
    /// Name of the TLS security policy that needs to be applied to the HTTPS endpoint. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_DomainEndpointOptions.html#opensearchservice-Type-DomainEndpointOptions-TLSSecurityPolicy). Pulumi will only perform drift detection if a configuration value is provided.
    #[builder(into, default)]
    #[serde(rename = "tlsSecurityPolicy")]
    pub r#tls_security_policy: Box<Option<String>>,
}