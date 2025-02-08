#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorityAccessUrl {
    /// (Output)
    /// The URL where this CertificateAuthority's CA certificate is published. This will only be
    /// set for CAs that have been activated.
    #[builder(into, default)]
    #[serde(rename = "caCertificateAccessUrl")]
    pub r#ca_certificate_access_url: Box<Option<String>>,
    /// (Output)
    /// The URL where this CertificateAuthority's CRLs are published. This will only be set for
    /// CAs that have been activated.
    #[builder(into, default)]
    #[serde(rename = "crlAccessUrls")]
    pub r#crl_access_urls: Box<Option<Vec<String>>>,
}
