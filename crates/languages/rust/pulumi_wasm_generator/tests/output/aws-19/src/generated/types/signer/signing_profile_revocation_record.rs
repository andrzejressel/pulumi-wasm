#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
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
