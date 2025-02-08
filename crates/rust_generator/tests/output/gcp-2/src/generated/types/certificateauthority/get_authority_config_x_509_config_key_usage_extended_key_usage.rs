#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAuthorityConfigX509ConfigKeyUsageExtendedKeyUsage {
    /// Corresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as "TLS WWW client authentication", though regularly used for non-WWW TLS.
    #[builder(into)]
    #[serde(rename = "clientAuth")]
    pub r#client_auth: Box<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as "Signing of downloadable executable code client authentication".
    #[builder(into)]
    #[serde(rename = "codeSigning")]
    pub r#code_signing: Box<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as "Email protection".
    #[builder(into)]
    #[serde(rename = "emailProtection")]
    pub r#email_protection: Box<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as "Signing OCSP responses".
    #[builder(into)]
    #[serde(rename = "ocspSigning")]
    pub r#ocsp_signing: Box<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as "TLS WWW server authentication", though regularly used for non-WWW TLS.
    #[builder(into)]
    #[serde(rename = "serverAuth")]
    pub r#server_auth: Box<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as "Binding the hash of an object to a time".
    #[builder(into)]
    #[serde(rename = "timeStamping")]
    pub r#time_stamping: Box<bool>,
}
