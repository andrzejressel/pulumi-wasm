#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateConfigSubjectKeyId {
    /// The value of the KeyId in lowercase hexadecimal.
    #[builder(into, default)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<Option<String>>,
}
