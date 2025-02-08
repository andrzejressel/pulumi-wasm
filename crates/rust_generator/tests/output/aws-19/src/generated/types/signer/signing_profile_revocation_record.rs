#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SigningProfileRevocationRecord {
    /// The time when revocation becomes effective.
    #[builder(into, default)]
    #[serde(rename = "revocationEffectiveFrom")]
    pub r#revocation_effective_from: Box<Option<String>>,
    /// The time when the signing profile was revoked.
    #[builder(into, default)]
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<Option<String>>,
    /// The identity of the revoker.
    #[builder(into, default)]
    #[serde(rename = "revokedBy")]
    pub r#revoked_by: Box<Option<String>>,
}
