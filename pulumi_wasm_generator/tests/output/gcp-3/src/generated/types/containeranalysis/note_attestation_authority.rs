#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NoteAttestationAuthority {
    /// This submessage provides human-readable hints about the purpose of
    /// the AttestationAuthority. Because the name of a Note acts as its
    /// resource reference, it is important to disambiguate the canonical
    /// name of the Note (which might be a UUID for security purposes)
    /// from "readable" names more suitable for debug output. Note that
    /// these hints should NOT be used to look up AttestationAuthorities
    /// in security sensitive contexts, such as when looking up
    /// Attestations to verify.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hint")]
    pub r#hint: Box<super::super::types::containeranalysis::NoteAttestationAuthorityHint>,
}
