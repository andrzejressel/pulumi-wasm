#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CaPoolPublishingOptions {
    /// Specifies the encoding format of each CertificateAuthority's CA
    /// certificate and CRLs. If this is omitted, CA certificates and CRLs
    /// will be published in PEM.
    /// Possible values are: `PEM`, `DER`.
    #[builder(into, default)]
    #[serde(rename = "encodingFormat")]
    pub r#encoding_format: Box<Option<String>>,
    /// When true, publishes each CertificateAuthority's CA certificate and includes its URL in the "Authority Information Access"
    /// X.509 extension in all issued Certificates. If this is false, the CA certificate will not be published and the corresponding
    /// X.509 extension will not be written in issued certificates.
    #[builder(into)]
    #[serde(rename = "publishCaCert")]
    pub r#publish_ca_cert: Box<bool>,
    /// When true, publishes each CertificateAuthority's CRL and includes its URL in the "CRL Distribution Points" X.509 extension
    /// in all issued Certificates. If this is false, CRLs will not be published and the corresponding X.509 extension will not
    /// be written in issued certificates. CRLs will expire 7 days from their creation. However, we will rebuild daily. CRLs are
    /// also rebuilt shortly after a certificate is revoked.
    #[builder(into)]
    #[serde(rename = "publishCrl")]
    pub r#publish_crl: Box<bool>,
}
