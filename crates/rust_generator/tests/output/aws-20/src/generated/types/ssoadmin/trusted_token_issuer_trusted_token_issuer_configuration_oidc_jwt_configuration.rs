#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration {
    /// Specifies the path of the source attribute in the JWT from the trusted token issuer.
    #[builder(into)]
    #[serde(rename = "claimAttributePath")]
    pub r#claim_attribute_path: Box<String>,
    /// Specifies path of the destination attribute in a JWT from IAM Identity Center. The attribute mapped by this JMESPath expression is compared against the attribute mapped by `claim_attribute_path` when a trusted token issuer token is exchanged for an IAM Identity Center token.
    #[builder(into)]
    #[serde(rename = "identityStoreAttributePath")]
    pub r#identity_store_attribute_path: Box<String>,
    /// Specifies the URL that IAM Identity Center uses for OpenID Discovery. OpenID Discovery is used to obtain the information required to verify the tokens that the trusted token issuer generates.
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Box<String>,
    /// The method that the trusted token issuer can use to retrieve the JSON Web Key Set used to verify a JWT. Valid values are `OPEN_ID_DISCOVERY`
    #[builder(into)]
    #[serde(rename = "jwksRetrievalOption")]
    pub r#jwks_retrieval_option: Box<String>,
}
