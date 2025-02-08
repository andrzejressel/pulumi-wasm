#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AttestorAttestationAuthorityNotePublicKeyPkixPublicKey {
    /// A PEM-encoded public key, as described in
    /// `https://tools.ietf.org/html/rfc7468#section-13`
    #[builder(into, default)]
    #[serde(rename = "publicKeyPem")]
    pub r#public_key_pem: Box<Option<String>>,
    /// The signature algorithm used to verify a message against
    /// a signature using this key. These signature algorithm must
    /// match the structure and any object identifiers encoded in
    /// publicKeyPem (i.e. this algorithm must match that of the
    /// public key).
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "signatureAlgorithm")]
    pub r#signature_algorithm: Box<Option<String>>,
}
