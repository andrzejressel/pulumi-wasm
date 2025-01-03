#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeyRingImportJobAttestation {
    /// (Output)
    /// The attestation data provided by the HSM when the key operation was performed.
    /// A base64-encoded string.
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// (Output)
    /// The format of the attestation data.
    #[builder(into, default)]
    #[serde(rename = "format")]
    pub r#format: Box<Option<String>>,
}
