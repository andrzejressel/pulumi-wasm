#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEncryption {
    /// An `identity` block as defined below.
    #[builder(into)]
    #[serde(rename = "identities")]
    pub r#identities: Box<Vec<super::super::types::loadtest::GetEncryptionIdentity>>,
    /// The URI specifying the Key vault and key to be used to encrypt data in this resource.
    #[builder(into)]
    #[serde(rename = "keyUrl")]
    pub r#key_url: Box<String>,
}
