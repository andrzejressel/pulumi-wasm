#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeystoresAliasesSelfSignedCertCertsInfoCertInfo {
    /// (Output)
    /// X.509 basic constraints extension.
    #[builder(into, default)]
    #[serde(rename = "basicConstraints")]
    pub r#basic_constraints: Box<Option<String>>,
    /// (Output)
    /// X.509 notAfter validity period in milliseconds since epoch.
    #[builder(into, default)]
    #[serde(rename = "expiryDate")]
    pub r#expiry_date: Box<Option<String>>,
    /// (Output)
    /// Flag that specifies whether the certificate is valid.
    /// Flag is set to Yes if the certificate is valid, No if expired, or Not yet if not yet valid.
    #[builder(into, default)]
    #[serde(rename = "isValid")]
    pub r#is_valid: Box<Option<String>>,
    /// (Output)
    /// X.509 issuer.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
    /// (Output)
    /// Public key component of the X.509 subject public key info.
    #[builder(into, default)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
    /// (Output)
    /// X.509 serial number.
    #[builder(into, default)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    /// (Output)
    /// X.509 signatureAlgorithm.
    #[builder(into, default)]
    #[serde(rename = "sigAlgName")]
    pub r#sig_alg_name: Box<Option<String>>,
    /// Subject details.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
    /// (Output)
    /// X.509 subject alternative names (SANs) extension.
    #[builder(into, default)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Box<Option<Vec<String>>>,
    /// (Output)
    /// X.509 notBefore validity period in milliseconds since epoch.
    #[builder(into, default)]
    #[serde(rename = "validFrom")]
    pub r#valid_from: Box<Option<String>>,
    /// (Output)
    /// X.509 version.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<i32>>,
}
