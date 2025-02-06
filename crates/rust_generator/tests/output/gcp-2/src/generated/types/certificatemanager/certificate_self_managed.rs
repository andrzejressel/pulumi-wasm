#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateSelfManaged {
    /// (Optional, Deprecated)
    /// The certificate chain in PEM-encoded form.
    /// Leaf certificate comes first, followed by intermediate ones if any.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    /// 
    /// > **Warning:** `certificate_pem` is deprecated and will be removed in a future major release. Use `pem_certificate` instead.
    #[builder(into, default)]
    #[serde(rename = "certificatePem")]
    pub r#certificate_pem: Box<Option<String>>,
    /// The certificate chain in PEM-encoded form.
    /// Leaf certificate comes first, followed by intermediate ones if any.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "pemCertificate")]
    pub r#pem_certificate: Box<Option<String>>,
    /// The private key of the leaf certificate in PEM-encoded form.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "pemPrivateKey")]
    pub r#pem_private_key: Box<Option<String>>,
    /// (Optional, Deprecated)
    /// The private key of the leaf certificate in PEM-encoded form.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    /// 
    /// > **Warning:** `private_key_pem` is deprecated and will be removed in a future major release. Use `pem_private_key` instead.
    #[builder(into, default)]
    #[serde(rename = "privateKeyPem")]
    pub r#private_key_pem: Box<Option<String>>,
}
