#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointAuthenticationOption {
    /// The ID of the Active Directory to be used for authentication if type is `directory-service-authentication`.
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryId")]
    pub r#active_directory_id: Box<Option<String>>,
    /// The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in AWS Certificate Manager (ACM). Only necessary when type is set to `certificate-authentication`.
    #[builder(into, default)]
    #[serde(rename = "rootCertificateChainArn")]
    pub r#root_certificate_chain_arn: Box<Option<String>>,
    /// The ARN of the IAM SAML identity provider if type is `federated-authentication`.
    #[builder(into, default)]
    #[serde(rename = "samlProviderArn")]
    pub r#saml_provider_arn: Box<Option<String>>,
    /// The ARN of the IAM SAML identity provider for the self service portal if type is `federated-authentication`.
    #[builder(into, default)]
    #[serde(rename = "selfServiceSamlProviderArn")]
    pub r#self_service_saml_provider_arn: Box<Option<String>>,
    /// The type of client authentication to be used. Specify `certificate-authentication` to use certificate-based authentication, `directory-service-authentication` to use Active Directory authentication, or `federated-authentication` to use Federated Authentication via SAML 2.0.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
