#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCryptoKeysKeyPrimary {
    /// The resource name for this CryptoKeyVersion.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The current state of the CryptoKeyVersion.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
