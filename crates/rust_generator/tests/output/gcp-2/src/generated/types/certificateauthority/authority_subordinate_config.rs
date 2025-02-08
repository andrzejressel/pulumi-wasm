#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthoritySubordinateConfig {
    /// This can refer to a CertificateAuthority that was used to create a
    /// subordinate CertificateAuthority. This field is used for information
    /// and usability purposes only. The resource name is in the format
    /// `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[builder(into, default)]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<Option<String>>,
    /// Contains the PEM certificate chain for the issuers of this CertificateAuthority,
    /// but not pem certificate for this CA itself.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pemIssuerChain")]
    pub r#pem_issuer_chain: Box<Option<super::super::types::certificateauthority::AuthoritySubordinateConfigPemIssuerChain>>,
}
