#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AttestorAttestationAuthorityNote {
    /// (Output)
    /// This field will contain the service account email address that
    /// this Attestor will use as the principal when querying Container
    /// Analysis. Attestor administrators must grant this service account
    /// the IAM role needed to read attestations from the noteReference in
    /// Container Analysis (containeranalysis.notes.occurrences.viewer).
    /// This email address is fixed for the lifetime of the Attestor, but
    /// callers should not make any other assumptions about the service
    /// account email; future versions may use an email based on a
    /// different naming pattern.
    #[builder(into, default)]
    #[serde(rename = "delegationServiceAccountEmail")]
    pub r#delegation_service_account_email: Box<Option<String>>,
    /// The resource name of a ATTESTATION_AUTHORITY Note, created by the
    /// user. If the Note is in a different project from the Attestor, it
    /// should be specified in the format `projects/*/notes/*` (or the legacy
    /// `providers/*/notes/*`). This field may not be updated.
    /// An attestation by this attestor is stored as a Container Analysis
    /// ATTESTATION_AUTHORITY Occurrence that names a container image
    /// and that links to this Note.
    #[builder(into)]
    #[serde(rename = "noteReference")]
    pub r#note_reference: Box<String>,
    /// Public keys that verify attestations signed by this attestor. This
    /// field may be updated.
    /// If this field is non-empty, one of the specified public keys must
    /// verify that an attestation was signed by this attestor for the
    /// image specified in the admission request.
    /// If this field is empty, this attestor always returns that no valid
    /// attestations exist.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "publicKeys")]
    pub r#public_keys: Box<Option<Vec<super::super::types::binaryauthorization::AttestorAttestationAuthorityNotePublicKey>>>,
}
