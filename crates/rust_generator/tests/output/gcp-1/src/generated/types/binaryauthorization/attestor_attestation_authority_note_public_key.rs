#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AttestorAttestationAuthorityNotePublicKey {
    /// ASCII-armored representation of a PGP public key, as the
    /// entire output by the command
    /// `gpg --export --armor foo@example.com` (either LF or CRLF
    /// line endings). When using this field, id should be left
    /// blank. The BinAuthz API handlers will calculate the ID
    /// and fill it in automatically. BinAuthz computes this ID
    /// as the OpenPGP RFC4880 V4 fingerprint, represented as
    /// upper-case hex. If id is provided by the caller, it will
    /// be overwritten by the API-calculated ID.
    #[builder(into, default)]
    #[serde(rename = "asciiArmoredPgpPublicKey")]
    pub r#ascii_armored_pgp_public_key: Box<Option<String>>,
    /// A descriptive comment. This field may be updated.
    #[builder(into, default)]
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    /// The ID of this public key. Signatures verified by BinAuthz
    /// must include the ID of the public key that can be used to
    /// verify them, and that ID must match the contents of this
    /// field exactly. Additional restrictions on this field can
    /// be imposed based on which public key type is encapsulated.
    /// See the documentation on publicKey cases below for details.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// A raw PKIX SubjectPublicKeyInfo format public key.
    /// NOTE: id may be explicitly provided by the caller when using this
    /// type of public key, but it MUST be a valid RFC3986 URI. If id is left
    /// blank, a default one will be computed based on the digest of the DER
    /// encoding of the public key.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pkixPublicKey")]
    pub r#pkix_public_key: Box<Option<super::super::types::binaryauthorization::AttestorAttestationAuthorityNotePublicKeyPkixPublicKey>>,
}
