#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAuthorityAccessUrl {
    /// The URL where this CertificateAuthority's CA certificate is published. This will only be
    /// set for CAs that have been activated.
    #[builder(into)]
    #[serde(rename = "caCertificateAccessUrl")]
    pub r#ca_certificate_access_url: Box<String>,
    /// The URL where this CertificateAuthority's CRLs are published. This will only be set for
    /// CAs that have been activated.
    #[builder(into)]
    #[serde(rename = "crlAccessUrls")]
    pub r#crl_access_urls: Box<Vec<String>>,
}
