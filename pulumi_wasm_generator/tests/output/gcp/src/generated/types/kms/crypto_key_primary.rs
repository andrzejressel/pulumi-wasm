#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CryptoKeyPrimary {
    /// The resource name for the CryptoKey.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// (Output)
    /// The current state of the CryptoKeyVersion.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
