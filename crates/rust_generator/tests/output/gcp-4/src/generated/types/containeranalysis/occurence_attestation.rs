#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OccurenceAttestation {
    /// The serialized payload that is verified by one or
    /// more signatures. A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "serializedPayload")]
    pub r#serialized_payload: Box<String>,
    /// One or more signatures over serializedPayload.
    /// Verifier implementations should consider this attestation
    /// message verified if at least one signature verifies
    /// serializedPayload. See Signature in common.proto for more
    /// details on signature structure and verification.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "signatures")]
    pub r#signatures: Box<Vec<super::super::types::containeranalysis::OccurenceAttestationSignature>>,
}
