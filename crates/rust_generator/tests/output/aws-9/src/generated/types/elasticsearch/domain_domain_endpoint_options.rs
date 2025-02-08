#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainDomainEndpointOptions {
    /// Fully qualified domain for your custom endpoint.
    #[builder(into, default)]
    #[serde(rename = "customEndpoint")]
    pub r#custom_endpoint: Box<Option<String>>,
    /// ACM certificate ARN for your custom endpoint.
    #[builder(into, default)]
    #[serde(rename = "customEndpointCertificateArn")]
    pub r#custom_endpoint_certificate_arn: Box<Option<String>>,
    /// Whether to enable custom endpoint for the Elasticsearch domain.
    #[builder(into, default)]
    #[serde(rename = "customEndpointEnabled")]
    pub r#custom_endpoint_enabled: Box<Option<bool>>,
    /// Whether or not to require HTTPS. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enforceHttps")]
    pub r#enforce_https: Box<Option<bool>>,
    /// Name of the TLS security policy that needs to be applied to the HTTPS endpoint. Valid values:  `Policy-Min-TLS-1-0-2019-07`, `Policy-Min-TLS-1-2-2019-07`, and `Policy-Min-TLS-1-2-PFS-2023-10`. Pulumi will only perform drift detection if a configuration value is provided.
    #[builder(into, default)]
    #[serde(rename = "tlsSecurityPolicy")]
    pub r#tls_security_policy: Box<Option<String>>,
}
