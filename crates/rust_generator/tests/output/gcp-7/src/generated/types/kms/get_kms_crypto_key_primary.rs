#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKmsCryptoKeyPrimary {
    /// The CryptoKey's name.
    /// A CryptoKey’s name belonging to the specified Google Cloud Platform KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The current state of the CryptoKeyVersion.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
