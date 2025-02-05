#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateTemplatePredefinedValuesKeyUsageBaseKeyUsage {
    /// The key may be used to sign certificates.
    #[builder(into, default)]
    #[serde(rename = "certSign")]
    pub r#cert_sign: Box<Option<bool>>,
    /// The key may be used for cryptographic commitments. Note that this may also be referred to as "non-repudiation".
    #[builder(into, default)]
    #[serde(rename = "contentCommitment")]
    pub r#content_commitment: Box<Option<bool>>,
    /// The key may be used sign certificate revocation lists.
    #[builder(into, default)]
    #[serde(rename = "crlSign")]
    pub r#crl_sign: Box<Option<bool>>,
    /// The key may be used to encipher data.
    #[builder(into, default)]
    #[serde(rename = "dataEncipherment")]
    pub r#data_encipherment: Box<Option<bool>>,
    /// The key may be used to decipher only.
    #[builder(into, default)]
    #[serde(rename = "decipherOnly")]
    pub r#decipher_only: Box<Option<bool>>,
    /// The key may be used for digital signatures.
    #[builder(into, default)]
    #[serde(rename = "digitalSignature")]
    pub r#digital_signature: Box<Option<bool>>,
    /// The key may be used to encipher only.
    #[builder(into, default)]
    #[serde(rename = "encipherOnly")]
    pub r#encipher_only: Box<Option<bool>>,
    /// The key may be used in a key agreement protocol.
    #[builder(into, default)]
    #[serde(rename = "keyAgreement")]
    pub r#key_agreement: Box<Option<bool>>,
    /// The key may be used to encipher other keys.
    #[builder(into, default)]
    #[serde(rename = "keyEncipherment")]
    pub r#key_encipherment: Box<Option<bool>>,
}
