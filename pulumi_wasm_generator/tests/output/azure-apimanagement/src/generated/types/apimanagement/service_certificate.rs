#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceCertificate {
    /// The password for the certificate.
    #[builder(into, default)]
    #[serde(rename = "certificatePassword")]
    pub r#certificate_password: Box<Option<String>>,
    /// The Base64 Encoded PFX or Base64 Encoded X.509 Certificate.
    #[builder(into)]
    #[serde(rename = "encodedCertificate")]
    pub r#encoded_certificate: Box<String>,
    /// The expiration date of the certificate in RFC3339 format: `2000-01-02T03:04:05Z`.
    #[builder(into, default)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<Option<String>>,
    /// The name of the Certificate Store where this certificate should be stored. Possible values are `CertificateAuthority` and `Root`.
    #[builder(into)]
    #[serde(rename = "storeName")]
    pub r#store_name: Box<String>,
    /// The subject of the certificate.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
    /// The thumbprint of the certificate.
    #[builder(into, default)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
}
