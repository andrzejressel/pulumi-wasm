#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InboundSamlConfigIdpConfig {
    /// The IdP's certificate data to verify the signature in the SAMLResponse issued by the IDP.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "idpCertificates")]
    pub r#idp_certificates: Box<Vec<super::super::types::identityplatform::InboundSamlConfigIdpConfigIdpCertificate>>,
    /// Unique identifier for all SAML entities
    #[builder(into)]
    #[serde(rename = "idpEntityId")]
    pub r#idp_entity_id: Box<String>,
    /// Indicates if outbounding SAMLRequest should be signed.
    #[builder(into, default)]
    #[serde(rename = "signRequest")]
    pub r#sign_request: Box<Option<bool>>,
    /// URL to send Authentication request to.
    #[builder(into)]
    #[serde(rename = "ssoUrl")]
    pub r#sso_url: Box<String>,
}
